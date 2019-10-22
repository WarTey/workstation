# Installation de l'active directory

Tutoriel réalisé par l'équipe workstation 2019-2020.

## Prérequis

1. Serveur Jarvis avec Proxmox installé.
2. ISO [Debian](https://www.debian.org/distrib/netinst).

## <a name="config-proxmox-ad">Configuration du serveur Proxmox pour l'active directory</a>

1. Connectez-vous à l'adresse du serveur Proxmox et ignorer le message qui s'affiche à l'écran en cliquant sur 'visiter ce site web'.

![Connectez-vous à l'adresse du serveur Proxmox et ignorer le message qui s'affiche à l'écran en cliquant sur 'visiter ce site web'.](https://raw.githubusercontent.com/WarTey/workstation/master/img/config_proxmox/capture_1.png)

2. Saisissez vos informations de connexion.

Rappel : le nom d'utilisateur est par défaut 'root'.

![Saisissez vos informations de connexion.](https://raw.githubusercontent.com/WarTey/workstation/master/img/config_proxmox/capture_2.png)

3. Ignorez le message d'erreur en cliquant sur 'OK'.

![Ignorez le message d'erreur en cliquant sur 'OK'.](https://raw.githubusercontent.com/WarTey/workstation/master/img/config_proxmox/capture_3.png)

4. Rendez-vous dans la partie (panneau gauche) 'Datacenter' -> 'pve' -> 'local (pve)' et sélectionnez 'Contenu'.

![Rendez-vous dans la partie (panneau gauche) 'Datacenter' -> 'pve' -> 'local (pve)' et sélectionnez 'Contenu'.](https://raw.githubusercontent.com/WarTey/workstation/master/img/config_proxmox/capture_4.png)

5. Cliquez sur 'Upload' puis choisissez votre ISO Debian et terminez en cliquant sur 'Upload'.

![Cliquez sur 'Upload' puis choisissez votre ISO Debian et terminez en cliquant sur 'Upload'.](https://raw.githubusercontent.com/WarTey/workstation/master/img/config_proxmox/capture_5.png)

6. Votre image ISO devrait maintenant appraître comme sur l'image ci-dessous.

![Votre image ISO devrait maintenant appraître comme sur l'image ci-dessous.](https://raw.githubusercontent.com/WarTey/workstation/master/img/config_proxmox/capture_6.png)

7. Cliquez maintenant sur 'Créer VM' (premier bouton bleu en haut à droite).

![Cliquez maintenant sur 'Créer VM' (premier bouton bleu en haut à gauche).](https://raw.githubusercontent.com/WarTey/workstation/master/img/config_proxmox/capture_7.png)

8. Dans l'onglet 'Général', choisissez le nom de votre machine virtuelle puis dans la partie 'Avancé' cochez 'Démarrer au boot'.

![Dans l'onglet 'Général', choisissez le nom de votre machine virtuelle puis dans la partie 'Avancé' cochez 'Démarrer au boot'.](https://raw.githubusercontent.com/WarTey/workstation/master/img/config_proxmox/capture_8.png)

9. Dans l'onglet 'OS', choisissez votre image ISO.

![Dans l'onglet 'OS', choisissez votre image ISO.](https://raw.githubusercontent.com/WarTey/workstation/master/img/config_proxmox/capture_9.png)

10. Dans l'onglet 'Disque Dur', choisissez 'SATA' dans 'Bus/Device'.

![Dans l'onglet 'Disque Dur', choisissez 'SATA' dans 'Bus/Device'.](https://raw.githubusercontent.com/WarTey/workstation/master/img/config_proxmox/capture_10.png)

11. Dans l'onglet 'Mémoire', choisissez '4096' dans 'Mémoire (MiB)'.

![Dans l'onglet 'Mémoire', choisissez '4096' dans 'Mémoire (MiB)'.](https://raw.githubusercontent.com/WarTey/workstation/master/img/config_proxmox/capture_11.png)

12. Dans l'onglet 'Confirmation', cliquez sur 'Terminé'.

![Dans l'onglet 'Confirmation', cliquez sur 'Terminé'.](https://raw.githubusercontent.com/WarTey/workstation/master/img/config_proxmox/capture_12.png)

13. Votre machine virtuelle devrait maintenant apparaître dans l'onglet de gauche sous 'pve'.

![Votre machine virtuelle devrait maintenant apparaître dans l'onglet de gauche sous 'pve'.](https://raw.githubusercontent.com/WarTey/workstation/master/img/config_proxmox/capture_13.png)

14. Cliquez sur cette dernière pour voir son tableau de bord.

![Cliquez sur cette dernière pour voir son tableau de bord.](https://raw.githubusercontent.com/WarTey/workstation/master/img/config_proxmox/capture_14.png)

## <a name="install-debian">Installation de Debian</a>

1. Une fois votre machine virtuelle créée dans Proxmox, cliquez sur 'Démarrer' (situé en haut à droite dans le résumé de votre machine virtuelle) et attendez de voir 'VM {ID} - Démarrer' dans les 'Tâches'.

![Une fois votre machine virtuelle créée dans Proxmox , cliquez sur 'Démarrer' (situé en haut à droite dans le résumé de votre machine virtuelle) et attendez de voir 'VM {ID} - Démarrer' dans les 'Tâches'.](https://raw.githubusercontent.com/WarTey/workstation/master/img/install_debian/capture_1.png)

2. Dans le tableau de bord de votre machine virtuelle sélectionnez 'Console' et vous devriez voir l'écran ci-dessous apparaître. Commencez une installation classique.

![Dans le tableau de bord de votre machine virtuelle sélectionnez 'Console' et vous devriez voir l'écran ci-dessous apparaître.](https://raw.githubusercontent.com/WarTey/workstation/master/img/install_debian/capture_2.png)

3. Pour la configuration du réseau, choisissez comme 'Nom de machine' : Debian.AD.

![Pour la configuration du réseau, choisissez comme 'Nom de machine' : Debian.AD.](https://raw.githubusercontent.com/WarTey/workstation/master/img/install_debian/capture_3.png)

4. Pour la création de l'utilisateur, choisissez comme 'Nom complet du nouvel utilisateur' : Debian AD.

![Pour la création de l'utilisateur, choisissez comme 'Nom complet du nouvel utilisateur' : Debian AD.](https://raw.githubusercontent.com/WarTey/workstation/master/img/install_debian/capture_4.png)

5. Pour la création de l'utilisateur, choisissez comme 'Identifiant pour le compte utilisateur' : ad.

![Pour la création de l'utilisateur, choisissez comme 'Identifiant pour le compte utilisateur' : ad.](https://raw.githubusercontent.com/WarTey/workstation/master/img/install_debian/capture_5.png)

6. Pour la sélection des logiciels, cochez seulement 'serveur SSH' et 'utilitaires usuels du système'.

![Pour la sélection des logiciels, cochez seulement 'serveur SSH' et 'utilitaires usuels du système'.](https://raw.githubusercontent.com/WarTey/workstation/master/img/install_debian/capture_6.png)

7. Une fois l'installation terminée, cliquez sur 'Continuer' et attendez que la machine virtuelle redémarre.

![Une fois l'installation terminée, cliquez sur 'Continuer' et attendez que la machine virtuelle redémarre.](https://raw.githubusercontent.com/WarTey/workstation/master/img/install_debian/capture_7.png)

## Configuration de l'active directory

1. Dans la machine virtuelle, connectez-vous avec l'utilisateur 'root'.

![Dans la machine virtuelle, connectez-vous avec l'utilisateur 'root'.](https://raw.githubusercontent.com/WarTey/workstation/master/img/config_active_directory/capture_1.png)

2. Définissez un 'hostname' nommé 'ad'.

```bash
hostnamectl set-hostname ad
```

3. Redémarrez la machine virtuelle.

```bash
reboot
```

4. Reconnectez-vous en tant que 'root'.

![Reconnectez-vous en tant que 'root'.](https://raw.githubusercontent.com/WarTey/workstation/master/img/config_active_directory/capture_1.png)

5. Installez les paquets ci-dessous.

```bash
apt install samba krb5-user krb5-config winbind libpam-winbind libnss-winbind -y
```

6. Sélectionnez 'Oui'.

![Sélectionnez 'Oui'.](https://raw.githubusercontent.com/WarTey/workstation/master/img/config_active_directory/capture_2.png)

7. Saisissez le royaume 'CYBER.LAN'. Attention aux majuscules.

![Saisissez le 'Royaume' : 'CYBER.LAN'. Attention aux majuscules.](https://raw.githubusercontent.com/WarTey/workstation/master/img/config_active_directory/capture_3.png)

7. Saisissez pour serveur kerberos 'cyber.lan'.

![Saisissez pour 'Serveur Kerberos' : 'cyber.lan'.](https://raw.githubusercontent.com/WarTey/workstation/master/img/config_active_directory/capture_4.png)

8. Saisissez pour serveur administratif 'cyber.lan'.

![Saisissez pour 'Serveur administratif' : 'cyber.lan'.](https://raw.githubusercontent.com/WarTey/workstation/master/img/config_active_directory/capture_5.png)

9. Arrêter et désactiver les services samba daemons.

```bash
systemctl stop samba-ad-dc.service smbd.service nmbd.service winbind.service
systemctl disable samba-ad-dc.service smbd.service nmbd.service winbind.service
```

10. Renommez le fichier de configuration samba.

```bash
mv /etc/samba/smb.conf /etc/samba/smb.conf.old
```

11. Utilisez la commande ci-dessous pour lancer la configuration du serveur samba.

```bash
samba-tool domain provision --use-rfc2307 --interactive
```

12. Saisissez comme 'Realm' : 'CYBER.LAN' puis continuez en appuyant 4 fois sur la touche 'Entrée' et choisissez un mot de passe pour 'Administrator'.

![Saisissez comme 'Realm' : 'CYBER.LAN' puis continuez en appuyant 4 fois sur la touche 'Entrée' et choisissez un mot de passe pour 'Administrator'.](https://raw.githubusercontent.com/WarTey/workstation/master/img/config_active_directory/capture_6.png)

13. Renommez le fichier de configuration krb5 et créer un lien symbolique sur ce dernier vers le répertoire '/etc'.

```bash
mv /etc/krb5.conf /etc/krb5.conf.old
ln -s /var/lib/samba/private/krb5.conf /etc/
```

14. Démarrez et activez les services samba daemons.

```bash
systemctl unmask samba-ad-dc.service
systemctl start samba-ad-dc.service
systemctl enable samba-ad-dc.service
```

15. Modifiez le fichier '/etc/network/interfaces' et '/etc/resolv.conf' comme ci-dessous.

```bash
nano /etc/network/interfaces
nano /etc/resolv.conf
```

![Modifiez le fichier '/etc/network/interfaces'.](https://raw.githubusercontent.com/WarTey/workstation/master/img/config_active_directory/capture_7.png)

![Modifiez le fichier '/etc/resolv.conf'.](https://raw.githubusercontent.com/WarTey/workstation/master/img/config_active_directory/capture_8.png)

16. Redémarrez la machine virtuelle.

```bash
reboot
```

17. Reconnectez-vous en tant que 'root'.

![Reconnectez-vous en tant que 'root'.](https://raw.githubusercontent.com/WarTey/workstation/master/img/config_active_directory/capture_1.png)

18. Exécutez les commandes suivantes.

```bash
host -t A cyber.lan
host -t A ad.cyber.lan
host -t SRV _kerberos._udp.cyber.lan
host -t SRV _ldap._tcp.cyber.lan
```

19. Redémarrez la machine virtuelle.

```bash
reboot
```

20. La configuration de l'active directory est maintenant terminée.

## Installation de SAMBA dans une VM extérieur

Le but de cette partie est d'installer un serveur SAMBA dans une VM séparée. La VM SAMBA regroupera les informations concernant les sessions de chaque utilisateurs ainsi que leur dossier personnel.

### Installation de la VM SAMBA dans Proxmox

Afin d'installer la VM dans Proxmox, il suffit de suivre la même procédure que celle décrite dans la section [Configuration du serveur Proxmox pour l'active directory](#config-proxmox-ad). 

Une fois la VM SAMVA ajoutée à Proxmox, il vous suffit d'installer Debian sur cette dernière en suivant les étapes de la section [Installation de Debian](#install-debian).

Une fois les deux étapes réalisées, SAMBA est prêt à être installé.

### Installation de SAMBA

1. Connectez-vous avec l'utilisateur 'root'.

2. Une fois connecter en root, lancer l'installation des paquets samba : 

```bash
apt install samba
```

### Configurer SAMBA



## Installation des scripts

Pour une meilleure maintenance du serveur, il est conseillé de créer un dossier 'scripts'. Ce dernier contiendra tous les fichiers automatisés et les scripts.

```mkdir ~/scripts```

Nous proposons un système permettant d'automatiser la gestion de l'inscription des utilisateurs dans l'AD. Ce qui signifie que les utilisateurs renseignés pourront recevoir un lien par mail, leurs permettant ainsi de choisir leur mot de passe en ligne (leur futur log-in). Nous expliquerons comment les scripts fonctionnent mais aussi comment héberger le site internet.

### Création d'un fichier de base de données utilisateurs

Pour utiliser notre système, il faut créer un fichier que nous nommerons étudiants. C'est dans ce dernier que vous devez renseigner tous les utilisateurs. (ne vous préocuppez pas des email pour le moment). 

```touch ~/scripts/etudiants```

```nano ~/scripts/etudiants```

Le format du fichier doit être le suivant : 

```bash 
nomutilisateur1,prenomutilisateur1
nomutilisateur2,prenomutilisateur2
...
```

> Si vous utilisez notre système pour l'ISEN, les noms/prénoms composés doivent être séparés d'un '-'.

Une fois ce fichier édité, créer un fichier generateUrlFromDatabase.sh puis éditez le.

```bash 
touch ~/scripts/generateUrlFromDatabase.sh 
nano ~/scripts/generateUrlFromDatabase.sh 
```
Ce fichier va permettre de générer des liens uniques pour chaque utilisateur et leur envoyer un mail avec ce dernier de sorte à ce qu'il puisse s'inscrire.

> Si vous utilisez notre système pour l'ISEN, nul besoin de changer les lignes ci-dessous. Si vous l'utilisez pou un autre établissement, il faut adapter les lignes.

```bash 
#!/bin/bash

urlNomDatabase(){
	numberLines=$(wc -l $1)
	uniquePassword=($(pwgen 13 $numberLines))
	iter=0
	> newDatabase
	while IFS= read -r line ; do
		#echo "Text read from file: $line"
		nom="$(echo $line | cut -d ',' -f1)"
		prenom="$(echo $line | cut -d ',' -f2)"
		#echo "On a prenom = $prenom et nom = $nom"
		email="$prenom.$nom@isen.yncrea.fr"
		#echo "l'adresse email est : $email"
		password=${uniquePassword[$iter]}
		echo "$prenom,$nom,$password" >> newDatabase
		cp emailTemplate email
		sed -i "s/*prenom/$prenom/g" email
		sed -i "s/*url/$password/g" email
		sendmail -f 'workstations@cybersecurite.com' $mail < email
		iter=$((iter + 1))
	done <"$1"
	chmod 777 newDatabase
}

if [[ ! $# -eq 0 ]]
then
	verif=$(urlNomDatabase $1)
	#echo $verif
else
	echo "Erreur! Chemin de la database attendu. Par défaut : '../database/etudiants'"
fi

```

Ce script va ainsi créer un fichier newDatabase contenant tous les utilisateurs et leurs clés/url respectives.
Vous noterez que pour utiliser ce script, on importe le contenu d'un fichier email, correspondant à notre template qui sera envoyé par mail. C'est l'objet de notre prochaine rubrique.
<<<<<<< HEAD
Aussi, il vous sera nécéssaire d'installer ces deux paquets avant d'éxécuter ce bash : 
>	```bash
>sudo apt install pwgen
>sudo apt install sendmail
>```	
=======

>>>>>>> dddf09bc2f86a7346f8d1c4592cb85128bffdf20
### Création d'un template d'email pour la notification

Toujours dans le dossier script, créer un fichier email et emailTemplate et éditer ce dernier. 

```bash 
touch ~/scripts/email 
touch ~/scripts/emailTemplate
nano ~/scripts/emailTemplate 
```

Vous collerez le message suivant, que vous pouvez bien évidemment personnaliser. 

```bash 
Subject : Votre mot de passe Mac

Bonjour *prenom
Voici le lien pour "setter" votre mot de passe. 
Attention, le lien n'est utilisable que pour votre user.

https://cyber.isen.fr?url=*url
```

Vous noterez que les '*' permettent de créer des variables, que nous remplissons avec le script generateUrlFromDatabase.sh 
Aussi, le fichier mail ne sert à rien d'autre que d'être rempli par notre template sans le modifier pour ainsi garder les variables.

### Ajouter des utilisateurs à notre AD

Le rôle de cette partie va en fait être de récupérer les données du site Web que nous allons importer plus tard : Voir [scripts#hebergement-web](https://github.com/WarTey/workstation/blob/master/active_directory.md#hebergement-web).
Ce dernier écrit simplement à la racine de l'hébergeur un fichier addUserToAD. Nous allons donc le lire depuis un script, pour ajouter nos nouveaux utilisateurs.
Toujours dans le dossier script, créer un fichier addUserToDatabase que nous éditerons. Ce fichier va permettre donc de lire les données du fichier web, pour ajouter l'utilisateur et le mot de passe correspondant à l'active directory.

```bash 
touch ~/scripts/addUserToDatabase 
nano ~/scripts/addUserToDatabase 
```

Insérez le contenu suivant dans le fichier.

```bash 
#!/bin/bash

file="/var/www/html/addUserToAD"

urlNomDatabase(){
	while IFS= read -r line ; do
		prenom="$(echo $line | cut -d ',' -f1)"
		nom="$(echo $line | cut -d ',' -f2)"
		password="$(echo $line | cut -d ',' -f3)"
		password=$(echo "$password" | base64 --decode)
		mkdir "/partage/$prenom.$nom" 
		samba-tool user create $prenom.$nom --given-name=$prenom --surname=$nom --mail-address=$prenom.$nom@isen.yncrea.fr --login-shell=/bin/bash "$password" ;
		samba-tool group addmembers Users $prenom.$nom
		done <"$file"
		rm "$file"
}

if [[ -f $file ]]
then
	#echo "le fichier existe"
	verif=$(urlNomDatabase)
	#echo $verif
else
	#echo "le fichier n'existe pas"
fi

```

Il faudrait maintenant faire en sorte que ce script se lance régulièrement, pour vérifier si le fichier est vide (personne ne s'est nouvellement inscrit) ou non vide (quelqu'un s'est inscrit).

On va, pour se faire, utiliser des crons jobs (à faire en root (su)).

```bash 
crontab -e
```

Et ajouter à la fin du fichier la ligne suivante, qui permettra d'éxécuter chaque minute notre script en fond. **Remplacez 'votreUser' par votre utilisateur.**

```bash 
* * * * * /home/votreUser/scripts/addUserToDatabase
```

Tout est prêt. Il ne vous reste plus qu'à importer le site web.

## Utilisation des scripts

### Pour générer les url et envoyer les mails

```bash 
bash ~/scripts/generateUrlFromDatabase.sh /chemin/vers/le/fichier/database
```

Aura donc pour effet de générer les url pour tous les utilisateurs présents et de leurs envoyer un mail avec leur lien.

### Facultatif: ajouter des utilisateurs sans cronjobs

Si pour une quelconque raison (cronjob qui ne vous convient pas) et que vous avez besoin d'éxécuter à la main l'ajout des utilisateurs fraichement inscrits : 

```bash 
bash ~/scripts/addUserToDatabase
```

## Hébergement Web

Quelques paquets sont nécéssaires pour l'hébergement web. 

```bash 
sudo apt update
sudo apt install apt-transport-https ca-certificates curl software-properties-common
curl -fsSL https://packages.sury.org/php/apt.gpg | sudo apt-key add -
sudo add-apt-repository "deb https://packages.sury.org/php/ $(lsb_release -cs) main"
sudo apt update
sudo apt install php7.2-common php7.2-cli
sudo apt install php7.2 libapache2-mod-php
sudo systemctl restart apache2
```

Ces commandes vous permettront d'avoir apache et php d'installer. Nous conseillons php7.2 mais libre à vous d'utiliser une autre version. 
Diriger vous maintenant dans le dossier apache par défaut et importer notre interface web.

Si git n'est pas encore installé : 

>	```bash
>sudo apt update
>sudo apt install git
>```

```bash
cd /var/www/html
rm -R '/var/www/html'/*
git init '/var/www/html/'
cd '/var/www/html/'
git remote add -f origin https://github.com/WarTey/workstation.git
git config core.sparseCheckout true
echo "web/" >> .git/info/sparse-checkout
git pull origin master
```

Ces commandes vont permettre de n'importer uniquement la partie web dans l'hébergeur.

Ensuite on va configurer apache pour rediriger les reqêtes HTML vers notre dossier. Pour ce faire on va utiliser les commandes suivantes.

```bash
nano /etc/apache2/apache2.conf
```

Vous pourrez ensuite aller à la ligne correspondant au <directory /var/www> pour le configurer comme ci-dessous.

![Configuration du Fichier apache2.conf](https://raw.githubusercontent.com/WarTey/workstation/master/img/config_active_directory_web/apache2.png)