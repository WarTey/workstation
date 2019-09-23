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

## Configuration de l'active directory