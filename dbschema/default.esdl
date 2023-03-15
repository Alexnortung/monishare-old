module default {
    type User {
        property username -> str;
        property email -> str;
        required property password_hash -> str;

        index on (.username);
        index on (.email);
    }

    type Project {
        required property name -> str;

        required link owner -> User;
        multi link members -> User;
    }

    type Message {
        required property text -> str;
        required property created_at -> datetime;

        required link author -> User;
        required link project -> Project;
    }

    type Post {
        property title -> str;
        required property amount -> decimal;

        required property created_at -> datetime;

        required link currency -> Currency;
        required link author -> User;
        required link project -> Project;

        link overridden_by -> Post;
    }

    type Currency {
        required property name -> str;
        required property code -> str;
    }
}
