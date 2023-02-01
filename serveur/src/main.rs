use std::net::TcpListener;
use std::thread;
use std::io::{ErrorKind, Read, Write};
use std::process;
use std::str;
use std::sync::mpsc;
use std::time::Duration;
use openssl::rsa::{Rsa, Padding};

// L'adresse et le port
const HOST: &str = "127.0.0.1:8080";

// Taille des messages
const MESSAGE: usize = 64;

/* 
struct Programme 
{
    nom:String
}

impl Programme 
{
    fn new(nom:String) -> Programme 
    {
        Programme {nom}
    }
    fn affichage_erreur(&self,msg: String)
    {
        println!("{}: Erreur rencontré : {}", self.nom, msg);
    }
    fn affichage_fail(&self, msg: String)
    {
        self.affichage_erreur(msg);
        self.fail();
    }
    fn exit(&self,statut: i32) -> ! 
    {
        process::exit(statut);
    }
    fn fail(&self) -> !  
    {
        self.exit(-1);
    }
}
*/

// fn encrypt_message(message: String) -> String {
//     let rsa = Rsa::generate(2048).expect("Erreur: impossible de générer la clé RSA");
//     let mut buf = vec![0; rsa.size() as usize];
//     let encrypted_len = rsa.public_encrypt(message.as_bytes(), &mut buf, Padding::PKCS1).unwrap();

//     encrypted_len.to_string()
// }

// La fonction sleep permet de notre thread de dormir un instant (100 milisecondes)
fn sleep()
{
    thread::sleep(Duration::from_millis(100));
}

fn main()
{
    //let programme = Programme::new("Programme Serveur".to_string());
    // Serveur
    let listener = match TcpListener::bind(HOST)
    {
        // Gestion d'erreur s'il n'arrive pas à se connecter
        Err(_) => 
        {
            println!("L'adresse est déjà utilisé");
            process::exit(0);
        },
        Ok(listener) => listener,  
    };
    // Nous mettons un drapeau non-bloquant du serveur en true
    listener.set_nonblocking(true).expect("Erreur d'initalisation de non-bloquant");
    
    println!("En attente des clients");
    
    // Créer un vecteur mutable pour les clients
    let mut clients = vec![];

    // On instance le channel et on l'affecte à un type String
    // Cela nous permettra d'envoyer et de recevoir des Strings via le channel
    let (sender, receiver) = mpsc::channel::<String>();

    loop
    {
        // On accepte les connexion à notre serveur. socket pour le flux TCP et addr pour l'adresse
        if let Ok((mut socket, addr)) = listener.accept()
        {
            println!("Connexion d'un client : {}", addr);

            // On clone l'envoyeur. Le socket va essayer de le cloner puis de le mettre vers le vecteur client
            let sender = sender.clone(); 
            clients.push(socket.try_clone().expect("Echec pour cloner le client"));   
            
            // On spawn le thread avec une fermeture à l'intérieur
            thread::spawn(move || loop 
            {
                //let programme = Programme::new("Client Thread".to_string());
                // On créer un buffer mutable
                let mut buffer = vec![0; MESSAGE];

                // Lis notre message dans notre buffer
                match socket.read_exact(&mut buffer)
                {
                    Ok(_) => 
                    {
                        // On récupère le message qu'on reçoit, on le convertit en itérateur.
                        // Nous prenons tous les caractères qui ne sont pas des espaces 
                        // On les rassembles dans un vecteur de sortie
                        // On convertit une tranche de String en une réelle String
                        let message = buffer.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let message = String::from_utf8(message).expect("Message utf8 invalide");

                        // encrypt_message(message);

                        let rsa = Rsa::generate(2048).expect("Erreur: impossible de générer la clé RSA");
                        let mut buf = vec![0; rsa.size() as usize];
                        let message = rsa.public_encrypt(message.as_bytes(), &mut buf, Padding::PKCS1).unwrap();

                        // println!("{message}");

                        // let message = message.to_string();
                        // let message = message.as_bytes();

                        // let message_decrypt = rsa.public_decrypt(message, &mut buf, Padding::PKCS1).unwrap();
                        
                        // On affiche l'adresse envoyée du message 
                        println!("{}: {:?}", addr, message);

                        

                        // Envoyer un message via notre envoyeur au réceptionneur
                        sender.send(message.to_string()).expect("Echec d'envoie du message");
                    },
                    // Si le type d'erreur est égal à une erreur qui bloquerait notre non-bloquant nous renvoyons le type d'unité
                    Err(ref erreur) if erreur.kind() == ErrorKind::WouldBlock => (),
                        
                    // Nous fermons la connexion, puis nous sortons de la boucle
                    Err(_) => 
                    {
                        println!("Fermeture de la connexion avec: {}", addr);
                        break;
                    } 
                }
                // Fait dormir le thread pendant 100 milisecondes. 
                // Elle permet à notre boucle de se reposer pendant qu'elle ne reçoit pas de messages
                sleep();
            });
        }    
            if let Ok(message) = receiver.try_recv()
            {
                clients = clients.into_iter().filter_map(|mut client| 
                {
                    // On définit le buffer égal au message qui est cloner et convertit en octets
                    let mut buffer = message.clone().into_bytes();

                    // Redimenssion du buffer en fonction de la taille du message
                    buffer.resize(MESSAGE, 0);

                    // On prend notre client, on écrit tout dans le buffer, on cartographie dans notre client
                    // On le renvoie puis on rassemble tout dans le vecteur
                    client.write_all(&buffer).map(|_| client).ok()
                }).collect::<Vec<_>>();
            }
                // Fait dormir le thread pendant 100 milisecondes
                sleep();
        }
}

