trait AbstractFactory {
    fn create_product_x(&self) -> ProductXEnum;
    fn create_product_y(&self) -> ProductYEnum;
}

trait ProductIf {
    fn get_value(&self) -> String;
}

enum ProductXEnum {
    ConcreteProductX1(ConcreteProductX1),
    ConcreteProductX2(ConcreteProductX2),
}

enum ProductYEnum {
    ConcreteProductY1(ConcreteProductY1),
    ConcreteProductY2(ConcreteProductY2),
}

impl ProductIf for ProductXEnum {
    fn get_value(&self) -> String {
        use ProductXEnum::*;
        match self {
            &ConcreteProductX1(ref x1) => x1.get_value(),
            &ConcreteProductX2(ref x2) => x2.get_value(),
        }
    }
}

impl ProductIf for ProductYEnum {
    fn get_value(&self) -> String {
        use ProductYEnum::*;
        match self {
            &ConcreteProductY1(ref y1) => y1.get_value(),
            &ConcreteProductY2(ref y2) => y2.get_value(),
        }
    }
}

struct ConcreteProductX1(String);
impl ConcreteProductX1 {
    fn new(msg: String) -> Self {
        ConcreteProductX1(msg + &" ProductX".to_string())
    }
}

impl ProductIf for ConcreteProductX1 {
    fn get_value(&self) -> String
    {
        self.0.clone()
    }
}

struct ConcreteProductX2(String);
impl ConcreteProductX2 {
    fn new(msg: String) -> Self {
        ConcreteProductX2(msg + &" ProductX".to_string())
    }
}

impl ProductIf for ConcreteProductX2 {
    fn get_value(&self) -> String
    {
        self.0.clone()
    }
}

struct ConcreteProductY1(String);
impl ConcreteProductY1 {
    fn new(msg: String) -> Self {
        ConcreteProductY1(msg + &" ProductY".to_string())
    }
}


impl ProductIf for ConcreteProductY1 {
    fn get_value(&self) -> String
    {
        self.0.clone()
    }
}

struct ConcreteProductY2(String);
impl ConcreteProductY2 {
    fn new(msg: String) -> Self {
        ConcreteProductY2(msg + &" ProductY".to_string())
    }
}


impl ProductIf for ConcreteProductY2 {
    fn get_value(&self) -> String
    {
        self.0.clone()
    }
}

struct ConcreteFactoryA;
impl AbstractFactory for ConcreteFactoryA {
    fn create_product_x(&self) -> ProductXEnum {
        ProductXEnum::ConcreteProductX1(ConcreteProductX1::new("FactoryB".to_string()))
    }

    fn create_product_y(&self) -> ProductYEnum {
        ProductYEnum::ConcreteProductY1(ConcreteProductY1::new("FactoryB".to_string()))
    }
}


struct ConcreteFactoryB;
impl AbstractFactory for ConcreteFactoryB {
    fn create_product_x(&self) -> ProductXEnum {
        ProductXEnum::ConcreteProductX2(ConcreteProductX2::new("FactoryB".to_string()))
    }

    fn create_product_y(&self) -> ProductYEnum {
        ProductYEnum::ConcreteProductY2(ConcreteProductY2::new("FactoryB".to_string()))
    }
}

enum FactoryID {
    A,
    B,
}

fn create_factory(id: FactoryID) -> Box<AbstractFactory> {
    match id {
        FactoryID::A => Box::new(ConcreteFactoryA),
        FactoryID::B => Box::new(ConcreteFactoryB),
    }
}

fn main() {
    let factory_a = create_factory(FactoryID::A);
    let a_x = factory_a.create_product_x();
    let a_y = factory_a.create_product_y();
    println!("{}", a_x.get_value());
    println!("{}", a_y.get_value());

    let factory_b = create_factory(FactoryID::B);
    let b_x = factory_b.create_product_x();
    let b_y = factory_b.create_product_y();
    println!("{}", b_x.get_value());
    println!("{}", b_y.get_value());
}