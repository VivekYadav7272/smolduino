/*
 * Scrapping this timer module. Feels like a _time_ ;) sink to implement.
 * Rather, I'll directly implement interrupts instead of this, like so:
 * struct Interrupt {
 *  trigger: InterruptTrigger,
 *  callback: impl Fn() -> () + 'static,
 * }
 * (something like this, not exact syntax)
 * in this way, I can probably abstract over needing a timer inside the InterruptTrigger (struct? enum?) and
 * special-case it.
*/
