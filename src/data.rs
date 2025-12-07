use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Apple iPhone 17 Pro Max 256GB - Cosmic Orange".to_string(),
            price: 1999.99,
            description: "The ultimate mobile experience with A18 Bionic chip, pro-grade camera system, and an all-new Cosmic Orange finish.".to_string(),
            image: "/iphone.png".to_string()
        },
        Product {
            id: 2,
            name: "Hisense 50\" QD6QF Series 4K UHD QLED Fire TV".to_string(),
            price: 599.99,
            description: "Stunning 4K QLED picture quality powered by Fire TV for seamless streaming and voice control.".to_string(),
            image: "/qled_tv.png".to_string()
        },
        Product {
            id: 3,
            name: "Samsung 65\" U8200F 4K UHD Smart TV".to_string(),
            price: 1199.99,
            description: "Crystal display and powerful 4K processor deliver vivid colours and incredible depth. Tizen OS for smart features.".to_string(),
            image: "/samsung_tv.png".to_string()
        },
        Product {
            id: 4,
            name: "LG 65\" QNED EVO AI QNED85 4K Mini-LED Smart TV".to_string(),
            price: 1799.99,
            description: "Precision dimming and Quantum Dot technology provide exceptional contrast and ultra-deep blacks. WebOS for fluid navigation.".to_string(),
            image: "/lg_qned.png".to_string()
        },
        Product {
            id: 5,
            name: "Sony PlayStation 5 Slim Console".to_string(),
            price: 649.99,
            description: "Experience lightning-fast loading with an ultra-high-speed SSD, deeper immersion with haptic feedback, and a new generation of incredible PlayStation games.".to_string(),
            image: "/ps5.png".to_string()
        }
    ]
}