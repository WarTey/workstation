# Installation de Rust et des dépendances utilisées

Tutoriel réalisé par l'équipe workstation 2019-2020.

## Prérequis

1. Machine Virtuelle avec Debian



## <a name="install_curl">Installation de Curl</a>

1. Entrez la commande suivante dans votre terminal afin d'installer *curl*.

   ```bash
   sudo apt-get install curl
   ```



## <a name="install_rust">Installation de Rust</a>

1. Une fois *curl* installé sur votre machine, vous pouvez commencer l'installation de *Rust* grâce à la commande suivante:

   ```bash
   curl https://sh.rustup.rs -sSf | sh
   ```

   

2. Une fois la commande lancée, une fenêtre comme celle ci-dessous devrait apparaître.

   ![Menu d'installation pour Rust](https://raw.githubusercontent.com/WarTey/workstation/master/img/install-rust/capture_1.png)

   

3. Tapez '*1*' pour procéder à l'installation.

   ![Fin de l'installation de Rust](https://raw.githubusercontent.com/WarTey/workstation/master/img/install-rust/capture_2.png)

   

4. Une fois l'installation fini, il vous faut ajouter le répertoire `bin` à *Cargo* en faisant la commande suivante:

   ```bash
   source $HOME/.cargo/env
   ```

   

5. Maintenant, vous devez vous assurer que votre shell utilisateur fonctionnera avec *Rust*, pour ce faire, il vous suffit de taper la commande suivante :

   ```bash
   source ~/.profile
   ```

   

6. Enfin, il vous suffit d'installer quelques dépendances indispensables.

   ```bash
   sudo apt-get install build-essential -y
   ```

   

7. Vérifiez la version de Rust.

   ```bash
   rustc --version
   ```

   

   ![Version de Rust](https://raw.githubusercontent.com/WarTey/workstation/master/img/install-rust/capture_3.png)

   

## <a name="test_rust">Test de Rust</a>

​	Maintenant que *Rust* est installé, nous allons tester notre installation afin d'être sûr que cette 	dernière 	à été effectuée correctement.



1. Créez un répertoire `rusttest`.

   ```bash
   mkdir rusttest
   ```

   

2. Placez-vous dans ce nouveau répertoire.

   ```bash
   cd rusttest
   ```

   

3. Créez un nouveau fichier *Rust* nommé `ruttest.rs`.

   ```bash
   nano rusttest.rs
   ```

   

4. Copiez la fonction suivante dans votre fichier `rusttest.rs`.

   ```rust
   fn main() {
       println!("Hello World !");
   }
   ```

   

5. Sauvegardez et fermez le fichier.

   

6. Créez exécutable *Rust*.

   ```bash
   rustc rusttest.rs
   ```

   

7. Un nouvel exécutable du nom de `rusttest` vient d'être créé.

   ![Affichage de l'exécutable](https://raw.githubusercontent.com/WarTey/workstation/master/img/install-rust/capture_4.png)

   

8. Lancer exécutable grâce à la commande suivante :

   ```bash
   ./rusttest
   ```

   

9. Le message `Hello World !` s'affiche.

   ![Lancement de l'exécutable](https://raw.githubusercontent.com/WarTey/workstation/master/img/install-rust/capture_5.png)



## <a name="cargo">Premiers pas avec Cargo</a>

​	*Cargo* est le gestionnaire de paquets de *Rust*.



1.  Création d'un nouveau package avec *Cargo*:

   ```bash
   cargo new hello_world
   ```

   Par défaut, *Cargo* créé une "binary application". Pour créer une librairie utilisez plutôt la commande:

   ```bash
   cargo new hello_world --lib
   ```

   

2. Allez dans le répertoire `hello_world` précédemment créé.

   ```bash
   cd hello_world
   ```

   

3. Listez les fichiers générés par *Cargo*.

   ```bash
   tree .
   ```

   

   Voici l'arborescence créée par *Cargo* :

   ![Arborescence créée par Cargo :](https://raw.githubusercontent.com/WarTey/workstation/master/img/install-rust/capture_6.png)

   

   Le fichier `Cargo.toml` est le **Manifest**, il contient toutes les métadonnées que *Cargo* utilise pour compiler le paquet.

   ​		

   ![Cargo.toml](https://raw.githubusercontent.com/WarTey/workstation/master/img/install-rust/capture_7.png)

   

   Le fichier `src/main.rs` devrait être muni de la fonction `main` qui affiche le message `Hello, world!`

   ​	

   ![src/main.rs](https://raw.githubusercontent.com/WarTey/workstation/master/img/install-rust/capture_8.png)

   

4. Compilez le `hello_world` généré par *Cargo*. Pour ce faire, positionné vous dans le répertoire `hello_world` :

   ```bash
   cd hello_world
   ```

   

   Puis entrez la commande suivante :

   ```bash
   cargo build
   ```

   

   ![cargo build](https://raw.githubusercontent.com/WarTey/workstation/master/img/install-rust/capture_9.png)

   

5. Ensuite, il ne vous reste plus qu'à exécuter le programme.

   ```bash
   ./target/debug/hello_world
   ```

   

   ![execution de hello_world](https://raw.githubusercontent.com/WarTey/workstation/master/img/install-rust/capture_10.png)

   

6. A noter qu'il existe une commande qui permet de *compiler* ainsi que d'*exécuter* le programme :

   ```bash
   cargo run
   ```

   

   ![cargo compilation et exécution](https://raw.githubusercontent.com/WarTey/workstation/master/img/install-rust/capture_11.png)

   

## <a name="install_rocket">Installation de Rocket</a>

​	*Rocket* est framework web pour *Rust*

1. Pour installer *Rocket* positionnez-vous dans le dossier `hello_world`.

   ```bash
   cd hello_world
   ```

2. Ensuite, ajoutez la ligne suivante à votre fichier Cargo.toml. 

   ```
   [dependencies]
   rocket = "0.4.4"
   ```

   ![cargo compilation et exécution](https://raw.githubusercontent.com/WarTey/workstation/master/img/install-rust/capture_12.png)



## <a name="install_tera">Installation de Tera</a>

​	*Tera* est un moteur de template pour *Rust*.

​	

1. Afin de l'installer, il vous suffit d'aller dans votre répertoire `hello_world`.

   ```bash
   cd hello_world
   ```
   
   
   
2. Puis d'ajouter la ligne suivante à votre `Cargo.toml` :

   ```rust
   [dependencies.rocket_contrib]
   version = "0.4.4"
   default-features = false
   features = ["tera_templates"]
   ```

   ![cargo compilation et exécution](https://raw.githubusercontent.com/WarTey/workstation/master/img/install-rust/capture_13.png)
