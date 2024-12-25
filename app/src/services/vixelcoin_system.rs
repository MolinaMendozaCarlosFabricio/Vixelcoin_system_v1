use sails_rs::{ prelude::*, collections::HashMap, gstd::{exec, msg} };

const DECIMALS: u128 = 1_000_000_000_000_000_000;
const VALUE_OF_VIXELCOIN: u128 = 1000; // Suponiendo, por ejemplo, que un vara equivale a 1000 vixelcoins


#[derive(Default)]
pub struct DataAcountUser{
    pub user_name: String,
    pub vixel_coins_amount: u128,
}

// Estado para el servicio de economía de vixel
#[derive(Default)]
pub struct VixelcoinSystemService{
    acount_users: HashMap<ActorId, DataAcountUser>
}

// Servicio para manejar la economía de vixel
#[sails_rs::service]
impl VixelcoinSystemService {

    pub fn new() -> Self {
        Self{
            acount_users: HashMap::<ActorId, DataAcountUser>::new(),
        }
    }

    // Métodos de prueba
    // Service's method (command)
    pub fn do_something(&mut self) -> String {
        "Hello from VixelcoinSystem!".to_string()
    }

    // Service's query
    pub fn get_something(&self) -> String {
        "Hello from VixelcoinSystem!".to_string()
    }

    // Los meros métodos
    // Método para registrar cuenta del usuario en el contrato
    pub fn register_user(&mut self, id_actor: ActorId, user_name: String) -> Result<(), &'static str>{
        // Comprueba que los campos estén completos
        if user_name.is_empty() {
            return Err("Campos faltantes");
        }

        // Crea el registro en el Estado
        self.acount_users.insert(id_actor, DataAcountUser{user_name, vixel_coins_amount: 0 });

        "User created with the Actor Id {ActorId}".to_string();
        Ok(())
    }

    // Método para cambiar varas por vixelcoin
    // #[payable]
    pub fn buy_vixelcoins(&mut self, id_actor: ActorId, amount: u128) -> Result<(), &'static str>{
        // Comprueba que el monto sea un valir válido
        if amount <= 0 {
            return Err("La cantidad debe ser mayor a cero");
        }
        // Obtiene directamente la cantidad de varas ingresado
        let amount_of_varas = (amount as u128) / DECIMALS;
        // Calcula su equivalencia en vixelcoin
        let amount_of_vixelcoins = Self::varas_to_vixelcoins(amount_of_varas);
        // Busca el usuario por medio del id Actor
        let user = self.acount_users.get(&id_actor).ok_or("El usuario no está registrado en el contrato")?;
        // Actualiza el registro con el usuario
        self.acount_users.insert(id_actor, DataAcountUser{ user_name: user.user_name.clone(), vixel_coins_amount: user.vixel_coins_amount + amount_of_vixelcoins});

        // msg::send(program, payload, value);
        
        "{amunt_of_vixelcoins} Vixelcoins para el usuario {id_actor}".to_string();
        Ok(())
    }

    // Método para vender vixelcoins por varas
    pub fn sell_vixelcoins(&mut self, id_actor: ActorId, amount_of_vixelcoins: u128) -> Result<(), &'static str>{
        // Lo pasa al formato del token de vara
        let amount_of_varas = Self::vixelcoins_to_varas(amount_of_vixelcoins) * DECIMALS;
        // Comprueba que el contrato tenga suficientes varas
        let contract_balance = exec::gas_available();
        if (contract_balance as u128) < amount_of_varas {
            return Err("El contrato no tiene suficientes varas");
        }

        // Obtiene un usuario por su id
        let user = self.acount_users.get(&id_actor).ok_or("No se ha registrado el usuario en el contrato")?;

        // Comprueba que el usuario contenga los suficientes vixelcoins
        if user.vixel_coins_amount < amount_of_vixelcoins {
            return Err("El usuario no cuenta con suficientes Vixelcoins para el cambio");
        }

        // Actualiza la cantidad de vixelcoins del usuario
        self.acount_users.insert(id_actor, DataAcountUser{ user_name: user.user_name.clone(), vixel_coins_amount: user.vixel_coins_amount - amount_of_vixelcoins});
        
        // Transfiere los varas al usuario
        let payload = "El usuario {ActorId} hizo la compra de {amount_of_vixelcoins} Tokens de Vara";
        msg::send(id_actor, payload, amount_of_varas).expect("Error al realizar la transacción");
        
        "{amount_of_varas} Tokens de Vara comprados por {id_actor}".to_string();
        Ok(())
    }

    /* 
    pub fn see_vixelcoins(& self, id_actor: ActorId) -> Result<DataAcountUser, &'static str>{
        let user = self.acount_users.get(&id_actor).ok_or("No se encontró información del usuario")?;
        event!("InfoAcountSend", { "address": id_actor, "name": user.user_name, "vixelcoins": user.vixel_coins_amount });
        Ok(user)
    }
    */

    fn varas_to_vixelcoins(varas: u128) -> u128 {
        varas * VALUE_OF_VIXELCOIN
    }
    
    fn vixelcoins_to_varas(vixelcoins: u128) -> u128 {
        vixelcoins / VALUE_OF_VIXELCOIN
    }
}