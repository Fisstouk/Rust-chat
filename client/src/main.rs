use std::net::TcpStream;
use std::thread;
use std::io::{self, ErrorKind, Read, Write};
use std::process;
use std::str;
use std::sync::mpsc::{self, TryRecvError};
use std::time::Duration;

// L'adresse et le port
const HOST: &str = "127.0.0.1:8080";

// Taille des messages
const MESSAGE: usize = 128; 
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
// La fonction sleep permet de notre thread de dormir un instant (100 milisecondes)
fn sleep()
{
    thread::sleep(Duration::from_millis(100));
}
fn pseudo() -> String
{
    println!("=============================");
    println!("========== BONJOUR ==========");
    println!("=============================\n");

    let mut user = String::new();
    println!("Veuillez entrer votre pseudo :");
    let stdin = io::stdin();
    stdin.read_line(&mut user);
    println!("Bonjour, {}", user);

    user
}
fn main() 
{
    let username = pseudo();
    //let programme = Programme::new("Programme Client".to_string());
    // Connexion du client mutable sur notre adresse IP indiqué dans HOST
    let mut client = match TcpStream::connect(HOST)
    {
        // Gestion d'erreur s'il n'arrive pas à se connecter
        Err(_) =>
        {
            println!("Erreur de connexion serveur");
            process::exit(0);
        },
        Ok(client) => client,
    };
    // Nous mettons un drapeau non-bloquant du client en true
    client.set_nonblocking(true).expect("Erreur d'initialisation du non-bloquant");
   
    // On instance le channel et on l'affecte à un type String
    // Cela nous permettra d'envoyer et de recevoir des Strings via le channel
    let (sender, receiver) = mpsc::channel::<String>();
    
    // On spawn un thread et on créer une fermeture de move à l'intérieur de la boucle
    thread::spawn(move || loop
    {
        // On créer un buffer mutable avec un vecteur avec des 0 dedans
        let mut client_buffer = vec![0; MESSAGE];
            
        // On lit le message via le buffer
        match client.read_exact(&mut client_buffer) 
        {
            Ok(_) => 
            {
                // Le message est égal au buffer, on le transforme en iterator,
                // On vérifie si les références sont égales à 0, on les collectes tous à l'intérieur du vecteur
                // Tous ceux qui sont égales à zéro seront défait
                let message = client_buffer.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                //let message = str::from_utf8(&message).unwrap();
                println!("Message :{:?}", message);
            },
            // Si le type d'erreur est égale à une erreur qui bloquerait notre non-bloquage, nous renvoyons alors le type d'unite
            Err(ref erreur) if erreur.kind() == ErrorKind::WouldBlock => (),

            // Si nous avons une erreur, nous fermerons la connexion puis nous sortirons de la boucle
            Err(_) =>
            {
                println!("Déconnexion du serveur");
                process::exit(0);
            }
        }
        match receiver.try_recv()
        {
            Ok(message) =>
            {
                // On clone le message en octets et on le met dans le buffer
                let mut client_buffer = message.clone().into_bytes();
                    
                // On redimenssionne le buffer par la taille du MESSAGE
                client_buffer.resize(MESSAGE, 0);
                    
                // On écrit tous les buffer dans notre client
                client.write_all(&client_buffer).expect("Echec d'écriture sur la socket");
                // Affiche notre message
            },
            // Vérifier si notre erreur est vide et si c'est le cas de renvoyer le type d'unité
            Err(TryRecvError::Empty) => (),
                
            // S'il s'agit d'une déconnexion alors on sort de la boucle
            Err(TryRecvError::Disconnected) => break
        }
        // Fait dormir le thread pendant 100 milisecondes
        sleep();
    });
    /*
    // Affichage lors de l'ouverture du client
    println!("=============================");
    println!("========== BONJOUR ==========");
    println!("=============================");
    */
    loop 
    {
        // Création d'une nouvelle String mutable
        let mut buffer = String::new();
        
        // On lit dans le String à partir de notre entrée standard
        io::stdin().read_line(&mut buffer).expect("Echec de lecture stdin");
        let mut full_message = username.clone();
        full_message.pop();

        // On coupe le buffer et on utilise le string pour la mettre dans une variable "message"
        let message = buffer.trim().to_string();
        full_message.push_str(" : ");
        full_message.push_str(&message); 
        
        // Si le message est "exit", alors on sort de la boucle
        if message == "exit" || sender.send(full_message).is_err() 
        {
            break
        }
    }
    // Affichage lors de la fermeture du client
    println!("===============================");
    println!("========== AU REVOIR ==========");
    println!("===============================");
}