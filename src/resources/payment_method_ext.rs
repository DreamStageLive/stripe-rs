use crate::config::{Client, Response};
use crate::ids::{CustomerId, PaymentMethodId};
use crate::params::Expand;
use crate::resources::PaymentMethod;
use serde_derive::{Deserialize, Serialize};

/// The parameters for `PaymentMethod::attach`
///
/// For more details see [https://stripe.com/docs/api/payment_methods/attach](https://stripe.com/docs/api/payment_methods/attach).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AttachPaymentMethod {
    pub customer: CustomerId,
}

/// The parameters for `PaymentMethod::detach`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct DetachPaymentMethod<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],
}
impl<'a> DetachPaymentMethod<'a> {
    pub fn new() -> Self {
        DetachPaymentMethod { expand: Default::default() }
    }
}

impl PaymentMethod {
    /// Attach a payment method to a customer
    ///
    /// For more details see [https://stripe.com/docs/api/payment_methods/attach](https://stripe.com/docs/api/payment_methods/attach).
    pub fn attach(
        client: &Client,
        payment_method_id: &PaymentMethodId,
        params: AttachPaymentMethod,
    ) -> Response<PaymentMethod> {
        client.post_form(&format!("/payment_methods/{}/attach", payment_method_id), params)
    }

    /// Detach a payment method from its customer
    ///
    /// For more details see [https://stripe.com/docs/api/payment_methods/detach](https://stripe.com/docs/api/payment_methods/detach).
    pub fn detach(
        client: &Client,
        id: &PaymentMethodId,
        params: DetachPaymentMethod<'_>,
    ) -> Response<PaymentMethod> {
        client.post_form(&format!("/payment_methods/{}/detach", id), &params)
    }
}
