use std::{net::SocketAddr, collections::hash_map::DefaultHasher};

use aquatic_udp_protocol::ConnectionId;

use super::{cypher::{BlowfishCypher, Cypher}, secret::Secret, timestamp_64::Timestamp64, client_id::Make, timestamp_32::Timestamp32, connection_id_data::ConnectionIdData, encrypted_connection_id_data::EncryptedConnectionIdData};

pub trait ConnectionIdIssuer {
    type Error;

    fn new_connection_id(&self, remote_address: &SocketAddr, current_timestamp: Timestamp64) -> ConnectionId;
    
    fn is_connection_id_valid(&self, connection_id: &ConnectionId, remote_address: &SocketAddr, current_timestamp: Timestamp64) -> bool;
}

/// An implementation of a ConnectionIdIssuer which encrypts the connection id
pub struct EncryptedConnectionIdIssuer {
    cypher: BlowfishCypher
}

impl ConnectionIdIssuer for EncryptedConnectionIdIssuer {
    type Error = &'static str;

    fn new_connection_id(&self, remote_address: &SocketAddr, current_timestamp: Timestamp64) -> ConnectionId {

        let connection_id_data = self.generate_connection_id_data(&remote_address, current_timestamp);

        let encrypted_connection_id_data = self.encrypt_connection_id_data(&connection_id_data);

        self.pack_connection_id(encrypted_connection_id_data)
    }

    fn is_connection_id_valid(&self, connection_id: &ConnectionId, remote_address: &SocketAddr, current_timestamp: Timestamp64) -> bool {

        let encrypted_connection_id_data: EncryptedConnectionIdData = self.unpack_connection_id(connection_id);

        let connection_id_data = self.decrypt_connection_id_data(&encrypted_connection_id_data);

        let current_client_id = Make::<DefaultHasher>::new(remote_address);
        if connection_id_data.client_id != current_client_id {
            return false;
        }

        if current_timestamp > connection_id_data.expiration_timestamp.into() {
            return false;
        }

        true
    }
}

impl EncryptedConnectionIdIssuer {
    pub fn new(secret: Secret) -> Self {
        let cypher = BlowfishCypher::new(secret);
        Self {
            cypher
        }
    }

    fn generate_connection_id_data(&self, remote_address: &SocketAddr, current_timestamp: Timestamp64) -> ConnectionIdData {
        let client_id = Make::<DefaultHasher>::new(remote_address);

        let expiration_timestamp: Timestamp32 = (current_timestamp + 120).try_into().unwrap();
    
        let connection_id_data = ConnectionIdData {
            client_id,
            expiration_timestamp
        };

        connection_id_data
    }

    fn pack_connection_id(&self, encrypted_connection_id_data: EncryptedConnectionIdData) -> ConnectionId {
        ConnectionId(encrypted_connection_id_data.into())
    }

    fn unpack_connection_id(&self, connection_id: &ConnectionId) -> EncryptedConnectionIdData {
        let encrypted_raw_data: EncryptedConnectionIdData = connection_id.0.into();
        encrypted_raw_data
    }

    fn decrypt_connection_id_data(&self, encrypted_connection_id_data: &EncryptedConnectionIdData) -> ConnectionIdData {
        let decrypted_raw_data = self.cypher.decrypt(&encrypted_connection_id_data.bytes);

        let connection_id_data = ConnectionIdData::from_bytes(&decrypted_raw_data);

        connection_id_data
    }

    fn encrypt_connection_id_data(&self, connection_id_data: &ConnectionIdData) -> EncryptedConnectionIdData {
        let decrypted_raw_data = connection_id_data.to_bytes();

        let encrypted_raw_data = self.cypher.encrypt(&decrypted_raw_data);

        let encrypted_connection_id_data = EncryptedConnectionIdData::from_encrypted_bytes(&encrypted_raw_data);

        encrypted_connection_id_data
    }
}

#[cfg(test)]
mod tests {
    use crate::udp::connection::{secret::Secret, connection_id_issuer::{EncryptedConnectionIdIssuer, ConnectionIdIssuer}};
    
    use std::{net::{SocketAddr, IpAddr, Ipv4Addr}};

    fn cypher_secret_for_testing() -> Secret {
        Secret::new([0u8;32])
    }

    fn new_issuer() -> EncryptedConnectionIdIssuer {
        let issuer = EncryptedConnectionIdIssuer::new(cypher_secret_for_testing());
        issuer
    }

    #[test]
    fn it_should_be_valid_for_two_minutes_after_the_generation() {
        let client_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let now = 946684800u64; // 01-01-2000 00:00:00

        let issuer = new_issuer();

        let connection_id = issuer.new_connection_id(&client_addr, now);

        assert!(issuer.is_connection_id_valid(&connection_id, &client_addr, now));

        let after_two_minutes = now + (2*60) - 1;

        assert!(issuer.is_connection_id_valid(&connection_id, &client_addr, after_two_minutes));
    }

    #[test]
    fn it_should_expire_after_two_minutes_from_the_generation() {
        let client_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let now = 946684800u64;

        let issuer = new_issuer();

        let connection_id = issuer.new_connection_id(&client_addr, now);

        let after_more_than_two_minutes = now + (2*60) + 1;

        assert_eq!(issuer.is_connection_id_valid(&connection_id, &client_addr, after_more_than_two_minutes), false);
    }    

    #[test]
    fn it_should_change_for_the_same_client_ip_and_port_after_two_minutes() {
        let client_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

        let now = 946684800u64;

        let issuer = new_issuer();

        let connection_id = issuer.new_connection_id( &client_addr, now);

        let after_two_minutes = now + 120;

        let connection_id_after_two_minutes = issuer.new_connection_id(&client_addr, after_two_minutes);

        assert_ne!(connection_id, connection_id_after_two_minutes);
    }

    #[test]
    fn it_should_be_different_for_each_client_at_the_same_time_if_they_use_a_different_ip() {
        let client_1_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)), 0001);
        let client_2_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0001);

        let now = 946684800u64;

        let issuer = new_issuer();

        let connection_id_for_client_1 = issuer.new_connection_id(&client_1_addr, now);
        let connection_id_for_client_2 = issuer.new_connection_id(&client_2_addr, now);

        assert_ne!(connection_id_for_client_1, connection_id_for_client_2);
    }

    #[test]
    fn it_should_be_different_for_each_client_at_the_same_time_if_they_use_a_different_port() {
        let client_1_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0001);
        let client_2_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0002);

        let now = 946684800u64;

        let issuer = new_issuer();

        let connection_id_for_client_1 = issuer.new_connection_id(&client_1_addr, now);
        let connection_id_for_client_2 = issuer.new_connection_id(&client_2_addr, now);

        assert_ne!(connection_id_for_client_1, connection_id_for_client_2);
    }

    #[test]
    fn it_should_fails_verifying_a_connection_id_when_the_client_id_in_the_connection_id_data_does_not_the_current_client_id() {
        let issuer = new_issuer();

        // Generate connection id for a given client
        let client_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0001);
        let now = 946684800u64;
        let connection_id = issuer.new_connection_id(&client_addr, now);

        // Verify the connection id with a different client address
        let different_client_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)), 0002);

        assert_eq!(issuer.is_connection_id_valid(&connection_id, &different_client_addr, now), false);
    }    
}