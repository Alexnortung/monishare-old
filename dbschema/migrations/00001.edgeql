CREATE MIGRATION m1owb37dnxo6ypx4vsmjbdh3qqo5m4gyb5ukxpnc4nbbqxgihanusq
    ONTO initial
{
  CREATE TYPE default::Currency {
      CREATE REQUIRED PROPERTY code -> std::str;
      CREATE REQUIRED PROPERTY name -> std::str;
  };
  CREATE TYPE default::User {
      CREATE PROPERTY username -> std::str;
      CREATE INDEX ON (.username);
      CREATE PROPERTY email -> std::str;
      CREATE INDEX ON (.email);
      CREATE REQUIRED PROPERTY password_hash -> std::str;
  };
  CREATE TYPE default::Project {
      CREATE MULTI LINK members -> default::User;
      CREATE REQUIRED LINK owner -> default::User;
      CREATE REQUIRED PROPERTY name -> std::str;
  };
  CREATE TYPE default::Post {
      CREATE REQUIRED LINK currency -> default::Currency;
      CREATE REQUIRED LINK author -> default::User;
      CREATE LINK overridden_by -> default::Post;
      CREATE REQUIRED LINK project -> default::Project;
      CREATE REQUIRED PROPERTY amount -> std::decimal;
      CREATE REQUIRED PROPERTY created_at -> std::datetime;
      CREATE PROPERTY title -> std::str;
  };
  CREATE TYPE default::Message {
      CREATE REQUIRED LINK author -> default::User;
      CREATE REQUIRED LINK project -> default::Project;
      CREATE REQUIRED PROPERTY created_at -> std::datetime;
      CREATE REQUIRED PROPERTY text -> std::str;
  };
};
