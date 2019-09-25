# Installation de l'active directory

Tutoriel réalisé par l'équipe workstation 2019-2020.

## Prérequis

1. Serveur Jarvis avec Proxmox installé.
2. ISO [Debian](https://www.debian.org/distrib/netinst).

## Configuration du serveur Proxmox pour l'active directory

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

7. Cliquez maintenant sur 'Créer VM' (premier bouton bleu en haut à gauche).

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

## Installation de Debian

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

19. Vérifiez l'identifiant 'Kerberos' en demandant un ticket avec les commandes ci-dessous.

```bash
kinit administrator@CYBER.LAN
klist
```

20. Redémarrez la machine virtuelle.

```bash
reboot
```

21. La configuration de l'active directory est maintenant terminée.