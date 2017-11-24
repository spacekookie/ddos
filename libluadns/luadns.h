/// DDOS native dns server module API

/** Start the dns server state with a given port */
int ddos_dns_start(int port);

/** Register the state of a DDOS application */
int ddos_register_state(const void *);

/** Register a single callback function */
int ddos_register_callback(void (*cb)(const void *));