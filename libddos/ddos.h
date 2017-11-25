/// DDOS native dns server module API

/** Start the dns server state with a given port */
int ddos_dns_start(int port);

/** Register the state of a DDOS application */
void ddos_register_state(const void *);

/** Register a single callback function */
void ddos_register_callback(int type, int* (*cb)(const void *, const char *));
