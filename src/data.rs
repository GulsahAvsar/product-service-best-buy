use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Samsung Galaxy S23 Ultra".to_string(),
            price: 1199.99,
            description: "Experience cutting-edge technology with the Samsung Galaxy S23 Ultra. Featuring a 200MP camera, stunning AMOLED display, and the latest Snapdragon processor.".to_string(),
            image: "/galaxy_s23_ultra.jpg".to_string(),
        },
        Product {
            id: 2,
            name: "Sony WH-1000XM5 Noise-Canceling Headphones".to_string(),
            price: 399.99,
            description: "Immerse yourself in high-fidelity sound with Sony's WH-1000XM5 headphones. Industry-leading noise cancellation and a lightweight design for ultimate comfort.".to_string(),
            image: "/sony_wh_1000xm5.jpg".to_string(),
        },
        Product {
            id: 3,
            name: "Apple MacBook Air M2".to_string(),
            price: 999.99,
            description: "The thinnest and lightest MacBook ever, powered by Apple's M2 chip. Offers incredible performance and battery life, perfect for students and professionals.".to_string(),
            image: "/macbook_air_m2.jpg".to_string(),
        },
        Product {
            id: 4,
            name: "Google Nest Hub (2nd Gen)".to_string(),
            price: 99.99,
            description: "Control your smart home and stay organized with the Google Nest Hub. Features a vibrant display and built-in Google Assistant.".to_string(),
            image: "/google_nest_hub.jpg".to_string(),
        },
        Product {
            id: 5,
            name: "Logitech MX Master 3S Wireless Mouse".to_string(),
            price: 99.99,
            description: "Enhance your productivity with the Logitech MX Master 3S. Ergonomic design, customizable buttons, and ultra-fast scrolling for an unmatched experience.".to_string(),
            image: "/mx_master_3s.jpg".to_string(),
        },
        Product {
            id: 6,
            name: "LG 55-Inch OLED TV (C3 Series)".to_string(),
            price: 1499.99,
            description: "Transform your living room with the stunning visuals of LG's OLED TV. Perfect blacks, vibrant colors, and a 120Hz refresh rate for an immersive experience.".to_string(),
            image: "/lg_oled_c3.jpg".to_string(),
        },
    ]
}
