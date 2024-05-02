use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub struct EmailAccount {
    email: String,
    password: String,
    services: HashMap<String, bool>,
}

impl EmailAccount {
    pub fn new(email: String, password: String) -> EmailAccount {
        EmailAccount {
            email,
            password,
            services: HashMap::new(),
        }
    }

    pub fn add_service(&mut self, service: String) {
        self.services.insert(service, true);
    }

    pub fn remove_service(&mut self, service: String) {
        self.services.remove(&service);
    }

    pub fn get_services(&self) -> Vec<String> {
        self.services.keys().map(|service| service.clone()).collect()
    }

    pub fn get_email(&self) -> String {
        self.email.clone()
    }

    pub fn get_password(&self) -> String {
        self.password.clone()
    }

}

#[derive(Serialize, Deserialize)]
pub struct EmailManager {
    accounts: HashMap<String, EmailAccount>,
}

impl EmailManager {
    pub fn new() -> EmailManager {
        EmailManager {
            accounts: HashMap::new(),
        }
    }

    pub fn load(path: &Path) -> EmailManager {
        if path.exists() {
            let mut file = File::open(path).unwrap();
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();
            serde_json::from_str(&data).unwrap()
        } else {
            EmailManager::new()
        }
    }

    pub fn save(&self, path: &Path) {
        let data = serde_json::to_string(self).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }

    pub fn add_account(&mut self, email: String, password: String) {
        self.accounts.insert(email.clone(), EmailAccount::new(email, password));
    }

    pub fn remove_account(&mut self, email: String) {
        self.accounts.remove(&email);
    }

    pub fn get_account(&mut self, email: String) -> Option<&mut EmailAccount> {
        self.accounts.get_mut(&email)
    }

    pub fn get_accounts(&self) -> Vec<EmailAccount> {
        self.accounts.values().map(|account| account.clone()).collect()
    }

    pub fn get_email_without_service(&self, service: String) -> Vec<EmailAccount> {
        self.accounts.iter().filter_map(|(_, account)| {
            if account.services.get(&service).is_none() {
                Some(account.clone())
            } else {
                None
            }
        }).collect()
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_account() {
        let mut account = EmailAccount::new("123456@gmail.com".to_string(), "password".to_string());
        account.add_service("keeta".to_string());
        account.add_service("gmail".to_string());
        account.remove_service("gmail".to_string());

        assert_eq!(account.email, "123456@gmail.com");
        assert_eq!(account.password, "password");
        assert_eq!(account.services.get("keeta"), Some(&true));
        assert_eq!(account.services.get("gmail"), None);
    }

    #[test]
    fn test_email_manager() {
        let mut manager = EmailManager::new();
        manager.add_account("123456@gmail.com".to_string(), "password".to_string() );
        manager.add_account("654321@outlook.com".to_string(), "password".to_string() );
        manager.get_account("123456@gmail.com".to_string())
            .unwrap()
            .add_service("keeta".to_string());
        manager.get_account("654321@outlook.com".to_string())
            .unwrap()
            .add_service("gmail".to_string());

        assert_eq!(manager.accounts.len(), 2);
        assert_eq!(manager.accounts.get("123456@gmail.com").unwrap().services.get("keeta"), Some(&true));

        manager.save(Path::new("test.json"));
        let manager2 = EmailManager::load(Path::new("test.json"));
        assert_eq!(manager2.accounts.len(), 2);
        assert_eq!(manager2.accounts.get("123456@gmail.com").unwrap().services.get("keeta"), Some(&true));

        let accounts = manager2.get_email_without_service("keeta".to_string());
        assert_eq!(accounts.len(), 1);
        assert!(accounts.iter().all(|account| account.email == "654321@outlook.com"));

    }
    
}
