use sails_rs::{
    prelude::*, 
    collections::HashMap, 
    gstd::{exec, msg}
};

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

    // Método para registrar cuenta del usuario en el contrato
    pub fn register_user(&mut self, user_name: String) -> Result<String, &'static str>{
        let id_actor = msg::source();
        // Comprueba que los campos estén completos
        if user_name.is_empty() {
            return Err("Campos faltantes");
        }
        
        let username: String = user_name.clone();
        // Crea el registro en el Estado
        self.acount_users.insert(id_actor, DataAcountUser{user_name: username, vixel_coins_amount: 0 });

        let user = self.acount_users.get(&id_actor).ok_or("Usuario no encontrado (¿Cómo es eso posible)")?;

        Ok(format!("Se creó el usuario {} con el ID {}", user.user_name, id_actor))
    }

    // Método para cambiar varas por vixelcoin
    // #[payable]
    pub fn buy_vixelcoins(&mut self) -> Result<String, &'static str>{
        // Obtiene información del mensaje
        let amount_varas = msg::value();
        let id_actor = msg::source();

        // Comprueba que el monto sea un valor válido
        if amount_varas <= 0 {
            return Err("La cantidad debe ser mayor a cero");
        }
        // Obtiene directamente la cantidad de varas ingresado
        let amount_of_varas = (amount_varas as u128) / DECIMALS;
        // Calcula su equivalencia en vixelcoin
        let amount_of_vixelcoins = Self::varas_to_vixelcoins(amount_of_varas);
        // Busca el usuario por medio del id Actor
        if !self.acount_users.contains_key(&id_actor) {
            return Err("El usuario no está registrado en el contrato");
        }

        self.acount_users.entry(id_actor).and_modify(
            |user| {
                user.vixel_coins_amount += amount_of_vixelcoins;
            }
        );
        
        let user = self.acount_users.get(&id_actor).ok_or("Usuario no encontrado")?;

        Ok(format!("El usuario {} compró {} Vixelcoins, su saldo actual es de {} Vixelcoins",
            user.user_name, amount_of_vixelcoins, user.vixel_coins_amount
        ))
    }

    // Método para vender vixelcoins por varas
    pub fn sell_vixelcoins(&mut self, amount_of_vixelcoins: u128) -> Result<String, &'static str>{
        let id_actor = msg::source();
        // Lo pasa al formato del token de vara
        let amount_of_varas = Self::vixelcoins_to_varas(amount_of_vixelcoins) * DECIMALS;
        // Comprueba que el contrato tenga suficientes varas
        let contract_balance = exec::gas_available();
        if (contract_balance as u128) < amount_of_varas {
            return Err("El contrato no tiene suficientes varas");
        }

        if !self.acount_users.contains_key(&id_actor) {
            return Err("El usuario no se encuentra registrado al contrato");
        }

        // Actualiza la cantidad de vixelcoins del usuario
        self.acount_users.entry(id_actor).and_modify(
            |user|{
                user.vixel_coins_amount -= amount_of_vixelcoins;
            }
        );

        // Obtiene un usuario por su id
        let user = self.acount_users.get(&id_actor).ok_or("No se ha registrado el usuario en el contrato")?;
        
        // Comprueba que el usuario contenga los suficientes vixelcoins
        if user.vixel_coins_amount < amount_of_vixelcoins {
            return Err("El usuario no cuenta con suficientes Vixelcoins para el cambio");
        }

        // Transfiere los varas al usuario
        let payload = "El usuario {ActorId} hizo la compra de {amount_of_vixelcoins} Tokens de Vara";
        msg::send(id_actor, payload, amount_of_varas).expect("Error al realizar la transacción");
        
        
        Ok(format!("El usuario {} ha comprado {} Varas, ahora cuenta con {} Vixecoins", 
            user.user_name, amount_of_varas, user.vixel_coins_amount
        ))
    }

    pub fn see_vixelcoins(& self) -> Result<String, &'static str>{
        let actor_id = msg::source();
        if !self.acount_users.contains_key(&actor_id) {
            return Err("No se encontró el usuario");
        }
        let user = self.acount_users.get(&actor_id).ok_or("No se econtró el usuario con ese id")?;
        Ok(format!("ID: {}, Name: {}, Vixelcoins: {}", actor_id, user.user_name, user.vixel_coins_amount))
    }

    fn varas_to_vixelcoins(varas: u128) -> u128 {
        varas * VALUE_OF_VIXELCOIN
    }
    
    fn vixelcoins_to_varas(vixelcoins: u128) -> u128 {
        vixelcoins / VALUE_OF_VIXELCOIN
    }
}