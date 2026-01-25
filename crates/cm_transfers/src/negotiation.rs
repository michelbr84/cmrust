//! Transfer negotiation (stub).

use cm_core::economy::Money;

/// Negotiation response.
#[derive(Debug)]
pub enum NegotiationResponse {
    Accept,
    Counter(Money),
    Reject,
}

/// Evaluate bid.
pub fn evaluate_bid(asking_price: Money, bid: Money, desperation: u8) -> NegotiationResponse {
    let ratio = bid.major() as f64 / asking_price.major().max(1) as f64;
    let threshold = 0.9 - (desperation as f64 / 500.0);

    if ratio >= 1.0 {
        NegotiationResponse::Accept
    } else if ratio >= threshold {
        NegotiationResponse::Counter(asking_price.multiply(0.95))
    } else {
        NegotiationResponse::Reject
    }
}
