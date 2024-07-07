#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait Contract {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[proxy]
    fn self_proxy(&self, remote_address: ManagedAddress) -> self::Proxy<Self::Api>;

    #[payable("*")]
    #[endpoint]
    fn call_remote(&self, remote_address: ManagedAddress) {
        let payment = self.call_value().egld_or_single_esdt();
        let caller = self.blockchain().get_caller();
        self.self_proxy(remote_address)
            .do_nothing()
            .callback(self.callbacks().call_remote_callback(caller, payment))
            .call_and_exit();
    }

    #[callback]
    fn call_remote_callback(
        &self,
        caller: ManagedAddress,
        payment: EgldOrEsdtTokenPayment<Self::Api>,
    ) {
        self.send().direct(&caller, &payment.token_identifier, payment.token_nonce, &payment.amount);
    }

    #[endpoint]
    fn do_nothing(&self) {}
}
