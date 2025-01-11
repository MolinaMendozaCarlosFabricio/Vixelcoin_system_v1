use sails_rs::{
    prelude::*, 
    collections::HashMap, 
    gstd::{exec, msg}
};

use crate::states::state::{
    VixelCoinSystemState,
    AcountInformation
};

const DECIMALS: u128 = 1_000_000_000_000_000_000;
const VALUE_OF_VIXELCOIN: u128 = 1000; // Suponiendo, por ejemplo, que un vara equivale a 1000 vixelcoins
static mut VIXELCOIN_SYSTEM_STATE: Option<VixelCoinSystemState> = None;


// Estado para el servicio de economía de vixel
#[derive(Default)]
pub struct VixelcoinSystemService{
    
}

// Servicio para manejar la economía de vixel
#[sails_rs::service]
impl VixelcoinSystemService {

    pub fn seed(adress: ActorId, vixelcoins: u128){
        unsafe {
            VIXELCOIN_SYSTEM_STATE = Some(
                VixelCoinSystemState {
                    admins: vec![adress],
                    amount_vixelcoins_total_in_the_system: Some(vixelcoins),
                    users: HashMap::<ActorId, AcountInformation>::new()
                }
            )
        }
    }

    pub fn new() -> Self {
        Self{}
    }

    // Método para registrar cuenta del usuario en el contrato
    pub fn register_user(&mut self, user_name: String) -> VixelcoinSystemEvents{
        let state = self.state_mut();
        let id_actor = msg::source();
        // Comprueba que los campos estén completos
        if user_name.is_empty() {
            return VixelcoinSystemEvents::Error(VixelCoinSystemErrors::WithoutSomeInputs);
        }

        if state.users.contains_key(&id_actor){
            return VixelcoinSystemEvents::Error(
                VixelCoinSystemErrors::UserExists
            );
        }
        
        // Crea el registro en el Estado
        state.users.entry(id_actor).insert(
            AcountInformation{username: user_name, balance_vixelcoins: 0}
        );

        let user = state.users.get(&id_actor).unwrap();

        VixelcoinSystemEvents::UserRegistred { 
            message: "Usuario registrado".to_string(), actor_id: id_actor, username: user.username.clone()
        }
    }

    // Método para cambiar varas por vixelcoin
    pub fn buy_vixelcoins(&mut self) -> VixelcoinSystemEvents{
        // Obtiene información del mensaje
        let amount_varas = msg::value();
        let id_actor = msg::source();
        let state = self.state_mut();

        // Comprueba que el monto sea un valor válido
        if amount_varas == 0 {
            return VixelcoinSystemEvents::Error(VixelCoinSystemErrors::MustBeGreaterThan0);
        }
        // Obtiene directamente la cantidad de varas ingresado
        let amount_of_varas = (amount_varas as u128) / DECIMALS;
        // Calcula su equivalencia en vixelcoin
        let amount_of_vixelcoins = Self::varas_to_vixelcoins(amount_of_varas);
        // Busca el usuario por medio del id Actor
        if !state.users.contains_key(&id_actor) {
            return VixelcoinSystemEvents::Error(VixelCoinSystemErrors::UserNotFound);
        }

        if state.amount_vixelcoins_total_in_the_system.is_none() {
            return VixelcoinSystemEvents::Error(VixelCoinSystemErrors::StateNotInicializated);
        }

        if state.amount_vixelcoins_total_in_the_system < Some(amount_of_vixelcoins) {
            return VixelcoinSystemEvents::Error(
                VixelCoinSystemErrors::InsuficentBalanceOfVixelcoinsInTheContract
            )
        }

        state.users.entry(id_actor).and_modify(
            |user| {
                user.balance_vixelcoins += amount_of_vixelcoins;
            }
        );

        // No sé si esto está haciendo efecto
        state.amount_vixelcoins_total_in_the_system = Some(state.amount_vixelcoins_total_in_the_system
            .unwrap()
            .saturating_sub(amount_of_vixelcoins));
        
        let user = state.users.get(&id_actor).unwrap();

        VixelcoinSystemEvents::VixecoinsBought { 
            message: "Vixelcoins comprados".to_string(), 
            actor_id: id_actor, username: user.username.clone(), vara_amount: amount_varas, 
            vixelcoin_bought: amount_of_vixelcoins, total_vixelcoin: user.balance_vixelcoins
        }
    }

    pub fn earn_award_of_vixelcoins(&mut self, amount_of_vixelcoins: u128) -> VixelcoinSystemEvents{
        let id_actor = msg::source();
        let state = self.state_mut();

        if !state.users.contains_key(&id_actor) {
            return VixelcoinSystemEvents::Error(
                VixelCoinSystemErrors::UserNotFound
            );
        }

        state.users.entry(id_actor).and_modify(
            |user|{
                user.balance_vixelcoins += amount_of_vixelcoins;
            }
        );

        if state.amount_vixelcoins_total_in_the_system.is_none() {
            return VixelcoinSystemEvents::Error(VixelCoinSystemErrors::StateNotInicializated);
        }

        if state.amount_vixelcoins_total_in_the_system < Some(amount_of_vixelcoins) {
            return VixelcoinSystemEvents::Error(
                VixelCoinSystemErrors::InsuficentBalanceOfVixelcoinsInTheContract
            )
        }

        state.amount_vixelcoins_total_in_the_system = Some(state.amount_vixelcoins_total_in_the_system
            .unwrap()
            .saturating_sub(amount_of_vixelcoins));

        VixelcoinSystemEvents::VixelcoinsEarned { 
            message: "Vixelcoins ganados".to_string(), 
            actor_id: id_actor, vixelcoins_amount: amount_of_vixelcoins 
        }
    }

    // Método para vender vixelcoins por varas
    pub fn sell_vixelcoins(&mut self, amount_of_vixelcoins: u128) -> VixelcoinSystemEvents{
        let state = self.state_mut();
        let id_actor = msg::source();
        // Lo pasa al formato del token de vara
        let amount_of_varas = Self::vixelcoins_to_varas(amount_of_vixelcoins) * DECIMALS;
        // Comprueba que el contrato tenga suficientes varas
        if exec::value_available() < amount_of_varas {
            return VixelcoinSystemEvents::Error(VixelCoinSystemErrors::InsuficentBalanceInTheContract(exec::value_available()));
        }

        if !state.users.contains_key(&id_actor) {
            return VixelcoinSystemEvents::Error(VixelCoinSystemErrors::UserNotFound);
        }

        // Obtiene un usuario por su id
        let user = state.users.get(&id_actor).unwrap();
        let balance_vixelcoins = user.balance_vixelcoins;
        let user_name = user.username.clone();
        
        // Comprueba que el usuario contenga los suficientes vixelcoins
        if balance_vixelcoins < amount_of_vixelcoins {
            // return Err("El usuario no cuenta con suficientes Vixelcoins para el cambio");
            return VixelcoinSystemEvents::Error(VixelCoinSystemErrors::InsuficentVixelcoins(balance_vixelcoins));
        }

        // Actualiza la cantidad de vixelcoins del usuario
        state.users.entry(id_actor).and_modify(
            |user|{
                user.balance_vixelcoins -= amount_of_vixelcoins;
            }
        );

        if state.amount_vixelcoins_total_in_the_system.is_none() {
            return VixelcoinSystemEvents::Error(VixelCoinSystemErrors::StateNotInicializated);
        }

        // No sé si esto está haciendo efecto
        state.amount_vixelcoins_total_in_the_system = Some(state.amount_vixelcoins_total_in_the_system
            .unwrap()
            .saturating_add(amount_of_vixelcoins));        

        // Transfiere los varas al usuario
        msg::send(
            id_actor, 
            "El usuario {ActorId} hizo la compra de {amount_of_vixelcoins} Tokens de Vara", 
            amount_of_varas
        )
            .expect("Error al realizar la transacción");
        
        VixelcoinSystemEvents::VarasBought { 
            message: "Vara Tokens bought".to_string(), actor_id: id_actor, username: user_name, 
            vixelcoin_amount: amount_of_vixelcoins, vara_bought: amount_of_varas, 
            total_vixelcoin: balance_vixelcoins
        }
    }

    pub fn spend_vixelcoins_in_the_system (&mut self, amount_of_vixelcoins: u128) -> VixelcoinSystemEvents {
        let state = self.state_mut();
        let id_actor = msg::source();

        if !state.users.contains_key(&id_actor) {
            return VixelcoinSystemEvents::Error(VixelCoinSystemErrors::UserNotFound)
        }

        let user = state.users.get(&id_actor).unwrap();

        if user.balance_vixelcoins < amount_of_vixelcoins {
            return VixelcoinSystemEvents::Error(VixelCoinSystemErrors::InsuficentVixelcoins(user.balance_vixelcoins));
        }

        state.users.entry(id_actor).and_modify(
            |edit_user|{
                edit_user.balance_vixelcoins -=amount_of_vixelcoins;
            }
        );

        if state.amount_vixelcoins_total_in_the_system.is_none() {
            return VixelcoinSystemEvents::Error(VixelCoinSystemErrors::StateNotInicializated);
        }

        // No sé si esto está haciendo efecto
        state.amount_vixelcoins_total_in_the_system = Some(state.amount_vixelcoins_total_in_the_system
            .unwrap()
            .saturating_add(amount_of_vixelcoins));

        VixelcoinSystemEvents::VixelcoinsSpended(amount_of_vixelcoins)
    }

    pub fn transfer_vixelcoins(&mut self, amount_of_vixelcoins: u128, destinatary: ActorId ) -> VixelcoinSystemEvents{
        let state = self.state_mut();
        let id_actor = msg::source();

        if !state.users.contains_key(&id_actor) || !state.users.contains_key(&destinatary) {
            return VixelcoinSystemEvents::Error(
                VixelCoinSystemErrors::UserNotFound
            );
        }

        let user = state.users.get(&id_actor).unwrap();

        if amount_of_vixelcoins == 0 {
            return  VixelcoinSystemEvents::Error(
                VixelCoinSystemErrors::MustBeGreaterThan0
            );
        }

        if user.balance_vixelcoins < amount_of_vixelcoins {
            return VixelcoinSystemEvents::Error(
                VixelCoinSystemErrors::InsuficentVixelcoins(user.balance_vixelcoins)
            );
        }

        state.users.entry(destinatary).and_modify(
            |user_destinatary|{
                user_destinatary.balance_vixelcoins += amount_of_vixelcoins;
            }
        );

        state.users.entry(id_actor).and_modify(
            |user_x|{
                user_x.balance_vixelcoins -= amount_of_vixelcoins;
            }
        );

        return VixelcoinSystemEvents::VarasTransfered { 
            message: "Vixelcoins transferidos".to_string(), 
            from: id_actor, to: destinatary, 
            vixelcoin_amount: amount_of_vixelcoins 
        }
    }

    pub fn see_vixelcoins_of_an_user(&self, adress: ActorId) -> VixelcoinSystemEvents{
        let state = self.state_mut();
        if !state.users.contains_key(&adress) {
            // return Err("No se encontró el usuario");
            return VixelcoinSystemEvents::Error(VixelCoinSystemErrors::UserNotFound);
        }
        let user = state.users.get(&adress).unwrap();
        // Ok(format!("ID: {}, Name: {}, Vixelcoins: {}", actor_id, user.user_name, user.vixel_coins_amount))
        VixelcoinSystemEvents::SeeUser { actor_id: adress, username: user.username.clone(), 
            total_vixelcoins: user.balance_vixelcoins
        }
    }

    pub fn see_vixelcoins_of_the_program(&self) -> VixelcoinSystemEvents{
        let state = self.state_mut();

        if !state.amount_vixelcoins_total_in_the_system.is_none(){
            return VixelcoinSystemEvents::Error(VixelCoinSystemErrors::StateNotInicializated);
        }

        VixelcoinSystemEvents::SeeBalanceOfTheProgram { 
            vixelcoins: state.amount_vixelcoins_total_in_the_system.unwrap(), 
            varas: exec::value_available() 
        }
    }

    pub fn add_vixelcoins_to_the_contract (&mut self, amount_of_vixelcoins: u128) -> VixelcoinSystemEvents {
        let state = self.state_mut();
        let id_actor = msg::source();

        if !state.users.contains_key(&id_actor) {
            return VixelcoinSystemEvents::Error(VixelCoinSystemErrors::UserNotFound);
        }

        if amount_of_vixelcoins == 0 {
            return VixelcoinSystemEvents::Error(VixelCoinSystemErrors::MustBeGreaterThan0);
        }

        if state.amount_vixelcoins_total_in_the_system.is_none() {
            return VixelcoinSystemEvents::Error(VixelCoinSystemErrors::StateNotInicializated);
        }

        // No sé si esto está haciendo efecto
        state.amount_vixelcoins_total_in_the_system = Some(state.amount_vixelcoins_total_in_the_system
            .unwrap()
            .saturating_add(amount_of_vixelcoins));

        VixelcoinSystemEvents::VixelcoinsAdded(amount_of_vixelcoins)
    }

    pub fn burn_vixelcoins_to_the_contract (&mut self, amount_of_vixelcoins: u128) -> VixelcoinSystemEvents{
        let state = self.state_mut();
        let id_actor = msg::source();

        if !state.users.contains_key(&id_actor) {
            return VixelcoinSystemEvents::Error(VixelCoinSystemErrors::UserNotFound);
        }

        if amount_of_vixelcoins == 0 {
            return VixelcoinSystemEvents::Error(VixelCoinSystemErrors::MustBeGreaterThan0);
        }

        if state.amount_vixelcoins_total_in_the_system.is_none() {
            return VixelcoinSystemEvents::Error(VixelCoinSystemErrors::StateNotInicializated);
        }

        state.amount_vixelcoins_total_in_the_system = Some(state.amount_vixelcoins_total_in_the_system
            .unwrap()
            .saturating_sub(amount_of_vixelcoins));

        VixelcoinSystemEvents::VixelcoinsBurned(amount_of_vixelcoins)
    }

    fn varas_to_vixelcoins(varas: u128) -> u128 {
        varas * VALUE_OF_VIXELCOIN
    }
    
    fn vixelcoins_to_varas(vixelcoins: u128) -> u128 {
        vixelcoins / VALUE_OF_VIXELCOIN
    }

    fn state_mut (&self) -> &'static mut VixelCoinSystemState {
        let state = unsafe {VIXELCOIN_SYSTEM_STATE.as_mut()};
        debug_assert!(state.is_none(), "El estado no ha sido inicializado");
        unsafe { state.unwrap_unchecked() }
    }

    fn state_ref (&self) -> &'static VixelCoinSystemState {
        let state = unsafe {VIXELCOIN_SYSTEM_STATE.as_ref()};
        debug_assert!(state.is_none(), "El estado no ha sido inicializado");
        unsafe { state.unwrap_unchecked() }
    }
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum VixelcoinSystemEvents {
    UserRegistred{
        message: String,
        actor_id: ActorId,
        username: String,
    },
    VixecoinsBought{
        message: String,
        actor_id: ActorId,
        username: String,
        vara_amount: u128,
        vixelcoin_bought: u128,
        total_vixelcoin: u128
    },
    VixelcoinsEarned{
        message: String,
        actor_id: ActorId,
        vixelcoins_amount: u128
    },
    VarasBought{
        message: String,
        actor_id: ActorId,
        username: String,
        vixelcoin_amount: u128,
        vara_bought: u128,
        total_vixelcoin: u128
    },
    VixelcoinsSpended(u128),
    VarasTransfered{
        message: String,
        from: ActorId,
        to: ActorId,
        vixelcoin_amount: u128
    },
    SeeUser{
        actor_id: ActorId,
        username: String,
        total_vixelcoins: u128
    },
    SeeBalanceOfTheProgram{
        vixelcoins: u128,
        varas: u128,
    },
    VixelcoinsAdded(u128),
    VixelcoinsBurned(u128),
    Error(VixelCoinSystemErrors)
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum VixelCoinSystemErrors {
    UserNotFound,
    UserExists,
    WithoutSomeInputs,
    MustBeGreaterThan0,
    InsuficentBalanceInTheContract(u128),
    InsuficentBalanceOfVixelcoinsInTheContract,
    InsuficentVixelcoins(u128),
    ErrorInTheTransaction,
    StateNotInicializated
}