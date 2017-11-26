// libddos is based on SimpleDNS which was 
//   initially written by Moritz Warning
// Repository can be found here: https://github.com/mwarning/SimpleDNS
// See the LICENSE file in this directory for
//   more information.

#include "ddos.h"
#include <lauxlib.h>
#include <stdlib.h>
#include <arpa/inet.h>
#include <string.h>
#include <errno.h>
#include <unistd.h>

#define BUF_SIZE 2048

struct IPAddress {
  int addr[16];
};

typedef struct IPAddress (*CallbackHandle)(void *, char *);

static void *state;
static CallbackHandle callbackA;
static CallbackHandle callbackAAAA;


static const uint32_t QR_MASK = 0x8000;
static const uint32_t OPCODE_MASK = 0x7800;
static const uint32_t AA_MASK = 0x0400;
static const uint32_t TC_MASK = 0x0200;
static const uint32_t RD_MASK = 0x0100;
static const uint32_t RA_MASK = 0x8000;
static const uint32_t RCODE_MASK = 0x000F;

enum {
  Ok_ResponseType = 0,
  FormatError_ResponseType = 1,
  ServerFailure_ResponseType = 2,
  NameError_ResponseType = 3,
  NotImplemented_ResponseType = 4,
  Refused_ResponseType = 5
};

enum {
  A_Resource_RecordType = 1,
  NS_Resource_RecordType = 2,
  CNAME_Resource_RecordType = 5,
  SOA_Resource_RecordType = 6,
  PTR_Resource_RecordType = 12,
  MX_Resource_RecordType = 15,
  TXT_Resource_RecordType = 16,
  AAAA_Resource_RecordType = 28,
  SRV_Resource_RecordType = 33
};

enum {
  QUERY_OperationCode = 0,
  IQUERY_OperationCode = 1,
  STATUS_OperationCode = 2,
  NOTIFY_OperationCode = 4,
  UPDATE_OperationCode = 5
};

enum {
  NoError_ResponseCode = 0,
  FormatError_ResponseCode = 1,
  ServerFailure_ResponseCode = 2,
  NameError_ResponseCode = 3
};

enum {
  IXFR_QueryType = 251,
  AXFR_QueryType = 252,
  MAILB_QueryType = 253,
  MAILA_QueryType = 254,
  STAR_QueryType = 255
};

/*
* Types.
*/

struct Question {
  char *qName;
  uint16_t qType;
  uint16_t qClass;
  struct Question* next; // for linked list
};

union ResourceData {
  struct {
    char *txt_data;
  } txt_record;
  struct {
    uint8_t addr[4];
  } a_record;
  struct {
    char* MName;
    char* RName;
    uint32_t serial;
    uint32_t refresh;
    uint32_t retry;
    uint32_t expire;
    uint32_t minimum;
  } soa_record;
  struct {
    char *name;
  } name_server_record;
  struct {
    char name;
  } cname_record;
  struct {
    char *name;
  } ptr_record;
  struct {
    uint16_t preference;
    char *exchange;
  } mx_record;
  struct {
    uint8_t addr[16];
  } aaaa_record;
  struct {
    uint16_t priority;
    uint16_t weight;
    uint16_t port;
    char *target;
  } srv_record;
};

struct ResourceRecord {
  char *name;
  uint16_t type;
  uint16_t class;
  uint16_t ttl;
  uint16_t rd_length;
  union ResourceData rd_data;
  struct ResourceRecord* next; // for linked list
};

struct Message {
  uint16_t id;

 
  uint16_t qr;
  uint16_t opcode;
  uint16_t aa;
  uint16_t tc;
  uint16_t rd;
  uint16_t ra;
  uint16_t rcode;

  uint16_t qdCount;
  uint16_t anCount;
  uint16_t nsCount;
  uint16_t arCount;
 
  struct Question* questions;
  struct ResourceRecord* answers;
  struct ResourceRecord* authorities;
  struct ResourceRecord* additionals;
};

char *to_nice_string(const char domain_name[]) {
  char *str = calloc(sizeof(char), strlen(domain_name) + 1);
  strcpy(str, domain_name);
  return str;
}

void my_string(void (*cb)(const char *)) {
  cb(to_nice_string("Does this work?"));
}

int get_A_Record(uint8_t addr[4], const char domain_name[], struct sockaddr_in* client_addr)
{
  char dom[strlen(domain_name) + 1];
  memcpy(dom, domain_name, strlen(domain_name));
  // int *address = callbackARecord(ddos_state, "to_nice_string(dom)");
  struct IPAddress address = callbackA(state, "Yes!");

  for(int i = 0; i < 4; i++) {
    addr[i] = (uint8_t) address.addr[i];
  }

  return 0;
}

int get_AAAA_Record(uint8_t addr[16], const char domain_name[], struct sockaddr_in* client_addr)
{
  char dom[strlen(domain_name) + 1];
  memcpy(dom, domain_name, strlen(domain_name));
  struct IPAddress address = callbackAAAA(state, to_nice_string(dom));

  for(int i = 0; i < 16; i++) {
    addr[i] = (uint8_t) address.addr[i];
  }

  return 0;
}

void print_hex(uint8_t* buf, size_t len)
{
  int i;
  printf("%u bytes:\n", len);
  for(i = 0; i < len; ++i)
    printf("%02x ", buf[i]);
  printf("\n");
}

void print_resource_record(struct ResourceRecord* rr)
{
  int i;
  while(rr)
  {
    printf("  ResourceRecord { name '%s', type %u, class %u, ttl %u, rd_length %u, ",
        rr->name,
        rr->type,
        rr->class,
        rr->ttl,
        rr->rd_length
    );

    union ResourceData *rd = &rr->rd_data;
    switch(rr->type)
    {
      case A_Resource_RecordType:
        printf("Address Resource Record { address ");
      
        for(i = 0; i < 4; ++i)
          printf("%s%u", (i ? "." : ""), rd->a_record.addr[i]);
      
        printf(" }");
        break;
      case NS_Resource_RecordType:
        printf("Name Server Resource Record { name %u}",
          rd->name_server_record.name
        );
        break;
      case CNAME_Resource_RecordType:
        printf("Canonical Name Resource Record { name %u}",
          rd->cname_record.name
        );
        break;
      case SOA_Resource_RecordType:
        printf("SOA { MName '%s', RName '%s', serial %u, refresh %u, retry %u, expire %u, minimum %u }",
          rd->soa_record.MName,
          rd->soa_record.RName,
          rd->soa_record.serial,
          rd->soa_record.refresh,
          rd->soa_record.retry,
          rd->soa_record.expire,
          rd->soa_record.minimum
        );
        break;
      case PTR_Resource_RecordType:
        printf("Pointer Resource Record { name '%s' }",
          rd->ptr_record.name
        );
        break;
      case MX_Resource_RecordType:
        printf("Mail Exchange Record { preference %u, exchange '%s' }",
          rd->mx_record.preference,
          rd->mx_record.exchange
        );
        break;
      case TXT_Resource_RecordType:
        printf("Text Resource Record { txt_data '%s' }",
          rd->txt_record.txt_data
        );
        break;
      case AAAA_Resource_RecordType:
        printf("AAAA Resource Record { address ");
      
        for(i = 0; i < 16; ++i)
          printf("%s%02x", (i ? ":" : ""), rd->aaaa_record.addr[i]);
      
        printf(" }");
        break;
      default:
        printf("Unknown Resource Record { ??? }");
    }
    printf("}\n");
    rr = rr->next;
  }
}

void print_query(struct Message* msg)
{
  printf("QUERY { ID: %02x", msg->id);
  printf(". FIELDS: [ QR: %u, OpCode: %u ]", msg->qr, msg->opcode);
  printf(", QDcount: %u", msg->qdCount);
  printf(", ANcount: %u", msg->anCount);
  printf(", NScount: %u", msg->nsCount);
  printf(", ARcount: %u,\n", msg->arCount);

  struct Question* q = msg->questions;
  while(q)
  {
    printf("  Question { qName '%s', qType %u, qClass %u }\n",
      q->qName,
      q->qType,
      q->qClass
    );
    q = q->next;
  }

  print_resource_record(msg->answers);
  print_resource_record(msg->authorities);
  print_resource_record(msg->additionals);

  printf("}\n");
}

size_t get16bits(const uint8_t** buffer)
{
  uint16_t value;

  value = ntohs( *((uint16_t*) *buffer) );
  *buffer += 2;

  return value;
}

void put8bits(uint8_t** buffer, uint8_t value)
{
  *((uint8_t*) *buffer) = value;
  *buffer += 1;
}

void put16bits(uint8_t** buffer, uint16_t value)
{
  *((uint16_t*) *buffer) = htons( value );
  *buffer += 2;
}

void put32bits(uint8_t** buffer, uint64_t value)
{
  *((uint64_t*) *buffer) = htonl( value );
  *buffer += 4;
}

// 3foo3bar3com0 => foo.bar.com
char* decode_domain_name(const uint8_t** buffer)
{
  uint8_t name[256];
  const uint8_t* buf = *buffer;
  int j = 0;
  int i = 0;
  while(buf[i] != 0) {
    //if(i >= buflen || i > sizeof(name))
    //  return NULL;
    
    if(i != 0) {
      name[j] = '.';
      j += 1;
    }

    int len = buf[i];
    i += 1;

    memcpy(name+j, buf+i, len);
    i += len;
    j += len;
  }

  name[j] = '\0';

  *buffer += i + 1;

  return strdup(name);
}

// foo.bar.com => 3foo3bar3com0
void encode_domain_name(uint8_t** buffer, const uint8_t* domain)
{
  uint8_t* buf = *buffer;
  const uint8_t* beg = domain;
  const uint8_t* pos;
  int len = 0;
  int i = 0;

  while(pos = strchr(beg, '.')) {
    len = pos - beg;
    buf[i] = len;
    i += 1;
    memcpy(buf+i, beg, len);
    i += len;

    beg = pos + 1;
  }

  len = strlen(domain) - (beg - domain);

  buf[i] = len;
  i += 1;

  memcpy(buf + i, beg, len);
  i += len;

  buf[i] = 0;
  i += 1;

  *buffer += i;
}


void decode_header(struct Message* msg, const uint8_t** buffer)
{
  msg->id = get16bits(buffer);

  uint32_t fields = get16bits(buffer);
  msg->qr = (fields & QR_MASK) >> 15;
  msg->opcode = (fields & OPCODE_MASK) >> 11;
  msg->aa = (fields & AA_MASK) >> 10;
  msg->tc = (fields & TC_MASK) >> 9;
  msg->rd = (fields & RD_MASK) >> 8;
  msg->ra = (fields & RA_MASK) >> 7;
  msg->rcode = (fields & RCODE_MASK) >> 0;

  msg->qdCount = get16bits(buffer);
  msg->anCount = get16bits(buffer);
  msg->nsCount = get16bits(buffer);
  msg->arCount = get16bits(buffer);
}

void encode_header(struct Message* msg, uint8_t** buffer)
{
  put16bits(buffer, msg->id);

  int fields = 0;
  fields |= (msg->qr << 15) & QR_MASK;
  fields |= (msg->rcode << 0) & RCODE_MASK;
  put16bits(buffer, fields);

  put16bits(buffer, msg->qdCount);
  put16bits(buffer, msg->anCount);
  put16bits(buffer, msg->nsCount);
  put16bits(buffer, msg->arCount);
}

int decode_msg(struct Message* msg, const uint8_t* buffer, int size)
{
  decode_header(msg, &buffer);

  if((msg->anCount+msg->nsCount) != 0) {
    printf("Only questions expected!\n");
    return -1;
  }

  uint32_t qcount = msg->qdCount;
  struct Question* qs = msg->questions;
  
  int i;
  for(i = 0; i < qcount; ++i) {
    struct Question* q = malloc(sizeof(struct Question));
    q->qName = decode_domain_name(&buffer);
    q->qType = get16bits(&buffer);
    q->qClass = get16bits(&buffer);

    q->next = qs; 
    msg->questions = q;
  }

  return 0;
}

void resolver_process(struct Message* msg,struct sockaddr_in* client_addr)
{
  struct ResourceRecord* beg;
  struct ResourceRecord* rr;
  struct Question* q;
  int rc;

  msg->qr = 1;
  msg->aa = 1;
  msg->ra = 0;
  msg->rcode = Ok_ResponseType;
  msg->anCount = 0;
  msg->nsCount = 0;
  msg->arCount = 0;

  q = msg->questions;
  while(q) {
    rr = malloc(sizeof(struct ResourceRecord));

    rr->name = strdup(q->qName);
    rr->type = q->qType;
    rr->class = q->qClass;
    rr->ttl = 60*60; //in seconds; 0 means no caching
    
    switch(q->qType) {
      case A_Resource_RecordType:
        rr->rd_length = 4;
        rc = get_A_Record(rr->rd_data.a_record.addr, q->qName, client_addr);
        if(rc < 0)
          goto next;
        break;
      case AAAA_Resource_RecordType:
        rr->rd_length = 16;
        rc = get_AAAA_Record(rr->rd_data.aaaa_record.addr, q->qName, client_addr);
        if(rc < 0)
          goto next;
        break;
      default:
        msg->rcode = NotImplemented_ResponseType;
        printf("Cannot answer question of type %d.\n", q->qType);
        goto next;
    }

    msg->anCount++;
    beg = msg->answers;
    msg->answers = rr;
    rr->next = beg;

    next: q = q->next;
  }
}

int encode_resource_records(struct ResourceRecord* rr, uint8_t** buffer)
{
  int i;
  while(rr) {
    // Answer questions by attaching resource sections.
    encode_domain_name(buffer, rr->name);
    put16bits(buffer, rr->type);
    put16bits(buffer, rr->class);
    put32bits(buffer, rr->ttl);
    put16bits(buffer, rr->rd_length);
    
    switch(rr->type)
    {
      case A_Resource_RecordType:
        for(i = 0; i < 4; ++i)
          put8bits(buffer, rr->rd_data.a_record.addr[i]);
        break;
      case AAAA_Resource_RecordType:
        for(i = 0; i < 16; ++i)
          put8bits(buffer, rr->rd_data.aaaa_record.addr[i]);
        break;
      default:
        fprintf(stderr, "Unknown type %u. => Ignore resource record.\n", rr->type);
      return 1;
    }
    
    rr = rr->next;
  }
  return 0;
}

int encode_msg(struct Message* msg, uint8_t** buffer)
{
  struct Question* q;
  int rc;

  encode_header(msg, buffer);

  q = msg->questions;
  while(q)
  {
    encode_domain_name(buffer, q->qName);
    put16bits(buffer, q->qType);
    put16bits(buffer, q->qClass);

    q = q->next;
  }

  rc = 0;
  rc |= encode_resource_records(msg->answers, buffer);
  rc |= encode_resource_records(msg->authorities, buffer);
  rc |= encode_resource_records(msg->additionals, buffer);

  return rc;
}

void free_resource_records(struct ResourceRecord* rr)
{
  struct ResourceRecord* next;

  while(rr) {
    free(rr->name);
    next = rr->next;
    free(rr);
    rr = next;
  }
}

void free_questions(struct Question* qq)
{
  struct Question* next;

  while(qq) {
    free(qq->qName);
    next = qq->next;
    free(qq);
    qq = next;
  }
}

void start_dns_server(int _port)
{

  // buffer for input/output binary packet
  uint8_t buffer[BUF_SIZE];
  struct sockaddr_in client_addr;
  socklen_t addr_len = sizeof(struct sockaddr_in);
  struct sockaddr_in addr;
  int nbytes, rc, buflen;
  int sock;
  int port = _port;

  struct Message msg;

  addr.sin_family = AF_INET;
  addr.sin_addr.s_addr = INADDR_ANY;
  addr.sin_port = htons(port);

  sock = socket(AF_INET, SOCK_DGRAM, 0);
  rc = bind(sock, (struct sockaddr*) &addr, addr_len);

  if(rc != 0) {
    printf("Could not bind: %s\n", strerror(errno));
    return 1;
  }

  printf("Listening on port %u.\n", port);

  while(1) {
    memset(&msg, 0, sizeof(struct Message));

    free_questions(msg.questions);
    free_resource_records(msg.answers);
    free_resource_records(msg.authorities);
    free_resource_records(msg.additionals);

    nbytes = recvfrom(sock, buffer, sizeof(buffer), 0,
      (struct sockaddr *) &client_addr, &addr_len);

    if(decode_msg(&msg, buffer, nbytes) != 0) {
      continue;
    }

    resolver_process(&msg,&client_addr);   

    uint8_t *p = buffer;
    if(encode_msg(&msg, &p) != 0) {
      continue;
    }

    int buflen = p - buffer;
    sendto(sock, buffer, buflen, 0, (struct sockaddr*) &client_addr, addr_len);
  }
}

/** Register a single callback function */
void ddos_register_callback(int type, int* (*cb)(const void *, const char *))
{
  printf("Regostering callback handle for type %d\n", type);
  switch(type) {
    case 4: 
      callbackA = cb;
      break;
    case 6: 
      callbackAAAA = cb;
      break;
    default:
      printf("Error: What are you on about? '%c'", type); 
      break;
  }
}


/********************************/

void set_state(void *s)
{
  state = s;
}

void set_callback(int type, int (*cb)(const void *, const char *))
{
  printf("Setting callback for %i\n", type);
  switch(type) {
    case 4: 
      callbackA = cb;
      break;
    case 6: 
      callbackAAAA = cb;
      break;
    default:
      printf("Error: What are you on about? '%c'", type); 
      break;
  }
}

void do_fun_stuff()
{
  printf("This is C\n");
  struct IPAddress addr = callbackA(state, "kookiejar.tech");
  printf("Return was %i\n", addr.addr[4]);
}