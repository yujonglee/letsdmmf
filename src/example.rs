use std::str::FromStr;

const ONE_TO_ONE: &str = include_str!("../examples/schema/1-1.prisma");
const ONE_TO_ONE_SELF: &str = include_str!("../examples/schema/1-1-self.prisma");
const ONE_TO_ONE_MULTI: &str = include_str!("../examples/schema/1-1-multi-field.prisma");
const ONE_TO_MANY: &str = include_str!("../examples/schema/1-n.prisma");
const ONE_TO_MANY_SELF: &str = include_str!("../examples/schema/1-n-self.prisma");
const ONE_TO_MANY_MULTI: &str = include_str!("../examples/schema/1-n-multi-field.prisma");
const MANY_TO_MANY_EXPLICIT: &str = include_str!("../examples/schema/m-n-explicit.prisma");
const MANY_TO_MANY_IMPLICIT: &str = include_str!("../examples/schema/m-n-implicit.prisma");
const MANY_TO_MANY_SELF: &str = include_str!("../examples/schema/m-n-self.prisma");
const MANY_TO_MANY_SELF_EXPLICIT: &str =
    include_str!("../examples/schema/m-n-self-explicit.prisma");

#[derive(Debug, PartialEq, Eq)]
pub enum Relation {
    OneToOne,
    OneToOneSelf,
    OneToOneMulti,
    OneToMany,
    OneToManySelf,
    OneToManyMulti,
    ManyToManyExplicit,
    ManyToManyImplicit,
    ManyToManySelf,
    ManyToManySelfExplicit,
}

impl Relation {
    pub fn read_schema(&self) -> String {
        match &self {
            Self::OneToOne => String::from(ONE_TO_ONE),
            Self::OneToOneSelf => String::from(ONE_TO_ONE_SELF),
            Self::OneToOneMulti => String::from(ONE_TO_ONE_MULTI),
            Self::OneToMany => String::from(ONE_TO_MANY),
            Self::OneToManySelf => String::from(ONE_TO_MANY_SELF),
            Self::OneToManyMulti => String::from(ONE_TO_MANY_MULTI),
            Self::ManyToManyExplicit => String::from(MANY_TO_MANY_EXPLICIT),
            Self::ManyToManyImplicit => String::from(MANY_TO_MANY_IMPLICIT),
            Self::ManyToManySelf => String::from(MANY_TO_MANY_SELF),
            Self::ManyToManySelfExplicit => String::from(MANY_TO_MANY_SELF_EXPLICIT),
        }
    }

    pub fn get_doc_url(&self) -> String {
        match &self {
            Self::OneToOne => String::from("https://www.prisma.io/docs/concepts/components/prisma-schema/relations/one-to-one-relations"),
            Self::OneToOneSelf  => String::from("https://www.prisma.io/docs/concepts/components/prisma-schema/relations/self-relations#one-to-one-self-relations"),
            Self::OneToOneMulti => String::from("https://www.prisma.io/docs/concepts/components/prisma-schema/relations/one-to-many-relations#multi-field-relations-in-relational-databases"),
            Self::OneToMany => String::from("https://www.prisma.io/docs/concepts/components/prisma-schema/relations/one-to-many-relations"),
            Self::OneToManySelf => String::from("https://www.prisma.io/docs/concepts/components/prisma-schema/relations/self-relations#one-to-many-self-relations"),
            Self::OneToManyMulti => String::from("https://www.prisma.io/docs/concepts/components/prisma-schema/relations/one-to-many-relations#multi-field-relations-in-relational-databases"),
            Self::ManyToManyExplicit => String::from("https://www.prisma.io/docs/concepts/components/prisma-schema/relations/many-to-many-relations#explicit-many-to-many-relations"),
            Self::ManyToManyImplicit => String::from("https://www.prisma.io/docs/concepts/components/prisma-schema/relations/many-to-many-relations#implicit-many-to-many-relations"),
            Self::ManyToManySelf => String::from("https://www.prisma.io/docs/concepts/components/prisma-schema/relations/self-relations#many-to-many-self-relations"),
            Self::ManyToManySelfExplicit => String::from("https://www.prisma.io/docs/concepts/components/prisma-schema/relations/self-relations#many-to-many-self-relations"),
        }
    }
}

impl FromStr for Relation {
    type Err = ();

    fn from_str(input: &str) -> Result<Relation, Self::Err> {
        match input {
            "1-1" => Ok(Self::OneToOne),
            "1-1-self" => Ok(Self::OneToOneSelf),
            "1-1-multi-field" => Ok(Self::OneToOneMulti),
            "1-n" => Ok(Self::OneToMany),
            "1-n-self" => Ok(Self::OneToManySelf),
            "1-n-multi-field" => Ok(Self::OneToManyMulti),
            "m-n-explicit" => Ok(Self::ManyToManyExplicit),
            "m-n-implicit" => Ok(Self::ManyToManyImplicit),
            "m-n-self" => Ok(Self::ManyToManySelf),
            "m-n-self-explicit" => Ok(Self::OneToOne),
            _ => Err(()),
        }
    }
}
