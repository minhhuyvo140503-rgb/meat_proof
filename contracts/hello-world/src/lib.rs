#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol, Vec
};

// --- Storage Keys and Data Structures ---

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StorageKey {

    Admin,                      // Instance storage: Admin Address

    Plan(u32),                  // Persistent storage: Subscription plan by ID

    Subscription(Address),     // Persistent storage: User's active subscription info

}



#[contracttype]

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct Plan {

    pub id: u32,

    pub name: Symbol,

    pub price: i128,            // Price in token base units

    pub duration: u64,          // Duration in seconds (timestamp delta)

}



#[contracttype]

#[derive(Clone, Debug, Eq, PartialEq)]

pub struct SubscriptionInfo {

    pub plan_id: u32,

    pub valid_until: u64,       // Unix timestamp cutoff

    pub is_active: bool,

    pub auto_renew: bool,

}



// --- Errors ---

#[contracttype]

#[derive(Copy, Clone, Debug, Eq, PartialEq)]

#[repr(u32)]

pub enum ContractError {

    NotAuthorized = 1,

    PlanNotFound = 2,

    SubscriptionExpired = 3,

    SubscriptionInactive = 4,

}



// --- Contract Definition ---



#[contract]

pub struct ChainSubscriptionHub;



#[contractimpl]

impl ChainSubscriptionHub {



    /// Khởi tạo contract và đặt địa chỉ Admin duy nhất quản lý các gói cước.

    pub fn initialize(env: Env, admin: Address) {

        if env.storage().instance().has(&StorageKey::Admin) {

            panic!("Contract da duoc khoi tao truoc do");

        }

        env.storage().instance().set(&StorageKey::Admin, &admin);

    }



    /// Thêm hoặc cập nhật một gói đăng ký mới (Chỉ Admin mới có quyền thực hiện).

    pub fn create_plan(env: Env, admin: Address, id: u32, name: Symbol, price: i128, duration: u64) {

        // Xác thực quyền Admin

        admin.require_auth();

        let stored_admin: Address = env.storage().instance().get(&StorageKey::Admin).unwrap();

        if admin != stored_admin {

            panic!("Khong co quyen Admin");

        }



        let plan = Plan { id, name, price, duration };

        env.storage().persistent().set(&StorageKey::Plan(id), &plan);

    }



    /// Người dùng đăng ký một gói cước cụ thể.

    pub fn subscribe(env: Env, user: Address, plan_id: u32, auto_renew: bool) {

        user.require_auth();



        // Kiểm tra xem Plan tồn tại hay không

        let plan_key = StorageKey::Plan(plan_id);

        if !env.storage().persistent().has(&plan_key) {

            panic!("Goi cuoc khong ton tai");

        }

        let plan: Plan = env.storage().persistent().get(&plan_key).unwrap();



        // Lấy thời gian block hiện tại (tính bằng giây)

        let current_time = env.ledger().timestamp();

        let valid_until = current_time + plan.duration;



        let sub_info = SubscriptionInfo {

            plan_id,

            valid_until,

            is_active: true,

            auto_renew,

        };



        env.storage().persistent().set(&StorageKey::Subscription(user), &sub_info);

    }



    /// Người dùng tự hủy kích hoạt hoặc tắt tính năng tự động gia hạn gói.

    pub fn cancel_subscription(env: Env, user: Address) {

        user.require_auth();

       

        let sub_key = StorageKey::Subscription(user.clone());

        if !env.storage().persistent().has(&sub_key) {

            panic!("Nguoi dung chua dang ky goi nao");

        }



        let mut sub_info: SubscriptionInfo = env.storage().persistent().get(&sub_key).unwrap();

        sub_info.auto_renew = false;

        sub_info.is_active = false; // Ngắt trạng thái active ngay lập tức hoặc khi hết hạn



        env.storage().persistent().set(&sub_key, &sub_info);

    }



    /// Kiểm tra trạng thái xem một người dùng có quyền truy cập dịch vụ hợp lệ hay không.

    pub fn check_status(env: Env, user: Address) -> bool {

        let sub_key = StorageKey::Subscription(user);

        if !env.storage().persistent().has(&sub_key) {

            return false;

        }



        let sub_info: SubscriptionInfo = env.storage().persistent().get(&sub_key).unwrap();

        let current_time = env.ledger().timestamp();



        // Đăng ký hợp lệ nếu thuộc trạng thái active và chưa quá hạn timestamp

        sub_info.is_active && (current_time <= sub_info.valid_until)

    }



    /// Truy vấn chi tiết thông tin gói cước.

    pub fn get_plan(env: Env, plan_id: u32) -> Option<Plan> {

        env.storage().persistent().get(&StorageKey::Plan(plan_id))

    }

}



// --- Unit Tests ---



#[cfg(test)]

mod test {

    use super::*;

    use soroban_sdk::Env;



    #[test]

    fn test_subscription_flow() {

        let env = Env::default();

        let contract_id = env.register_contract(None, ChainSubscriptionHub);

        let client = ChainSubscriptionHubClient::new(&env, &contract_id);



        // Tạo tài khoản giả lập cho Admin và Người dùng

        let admin = Address::generate(&env);

        let user = Address::generate(&env);



        // 1. Khởi tạo contract

        client.initialize(&admin);



        // 2. Admin tạo gói Premium (ID: 1, Giá: 1000, Thời hạn: 30 ngày = 2592000 giây)

        let plan_name = symbol_short!("Premium");

        client.create_plan(&admin, &1, &plan_name, &1000, &2592000);



        // Kiểm tra xem gói đã lưu thành công chưa

        let plan = client.get_plan(&1).unwrap();

        assert_eq!(plan.id, 1);

        assert_eq!(plan.price, 1000);



        // 3. Người dùng đăng ký gói cước

        // Thiết lập thời gian ledger ban đầu là 10,000 để dễ tính toán

        env.ledger().set_timestamp(10000);

        client.subscribe(&user, &1, &true);



        // Trạng thái kiểm tra phải thành công (Active = true và thời gian hiện tại 10000 <= 10000 + 2592000)

        assert!(client.check_status(&user));



        // 4. Giả lập thời gian trôi qua vượt quá thời hạn (10000 + 2592000 + 1)

        env.ledger().set_timestamp(10000 + 2592000 + 1);

       

        // Trạng thái kiểm tra phải trả về false do đã hết hạn

        assert!(!client.check_status(&user));

    }

}
