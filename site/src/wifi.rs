use ldap3::result::Result;
use ldap3::controls::{MakeCritical, RelaxRules};
use ldap3::{LdapConn, Mod};
use maplit::hashset;
use std::env;

pub fn wifi(id: String, passwd: String){
    let user = id.clone();
    let pwd = passwd.clone();
    update(id, passwd);
    add_user(user, pwd);
}

fn update(id: String, passwd: String) -> Result<()> {
    /* Initialisation de la connexion */
    let mut ldap = LdapConn::new("ldap://localhost:3890")?;

    /* Conversion des String en &str */
    let user_id: &str = &id;
    let user_password: &str = &passwd;

    /* Création du path de l'utilisateur */
    let debut: &str = "uid=";
    let fin: &str = ",ou=Eleves,dc=cyber,dc=isen,dc=fr";
    let together = format!("{}{}{}", debut, user_id, fin);

    /* Identification en tant qu'administrateur */
    let wifi_password = env::var("WIFI_PASSWORD").expect("WIFI_PASSWORD must be set");
    ldap.simple_bind("cn=admin,dc=cyber,dc=isen,dc=fr", &wifi_password)?
        .success()?;

    /* Verificvation de l'unicité de l'utilisateur */
    if ldap.compare(&together, "cn", &user_id)?.equal()? {
        ldap.with_controls(RelaxRules.critical())
            .modify(&together, vec![Mod::Replace("userPassword", hashset! {user_password})])?
            .success()?;
    }

    /* On détache la connexion */
    ldap.unbind()
}

fn add_user(id: String, passwd: String) -> Result<()> {
    /* Initialisation de la connexion */
    let mut ldap = LdapConn::new("ldap://localhost:3890")?;

    /* Conversion des String en str */
    let user_id: &str = &id;
    let user_password: &str = &passwd;

    /* Création du path de l'utilisateur */
    let debut: &str = "uid=";
    let fin: &str = ",ou=Eleves,dc=cyber,dc=isen,dc=fr";
    let together = format!("{}{}{}", debut, user_id, fin);

    /* Identification en tant qu'administrateur */
    let wifi_password = env::var("WIFI_PASSWORD").expect("WIFI_PASSWORD must be set");
    ldap.simple_bind("cn=admin,dc=cyber,dc=isen,dc=fr", &wifi_password)?
        .success()?;

    /* Ajout d'un user s'il n'existe pas */
    ldap.add(&together, vec![
        ("objectClass", hashset! {"inetOrgPerson"}),
        ("uid", hashset! {user_id}),
        ("cn", hashset! {user_id}),
        ("sn", hashset! {user_id}),
        ("userPassword", hashset! {user_password})
    ])?.success()?;

    /* On détache la connexion */
    ldap.unbind()
}