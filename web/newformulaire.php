<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>Formulaire Cyber</title>
        <link href="bootstrap/css/bootstrap.css" rel="stylesheet">
    </head>

    <body>
    <?php
    $prenom = "Sylvain" ;
    $nom = "Franco" ;

    ?>
        <!-- Default form register -->
        <form class="text-center border border-light p-5" method="post" action=cible.php>

        <p class="h4 mb-4">Formulaire Cyber</p>

        <div class="form-row mb-4">
            <div class="col">
                <!-- First name -->
                <input type="text" id="prenom" class="form-control" placeholder="Prenom" value=<?php echo $prenom ?> readonly="">
            </div>
            <div class="col">
                <!-- Last name -->
                <input type="text" id="nom" class="form-control" placeholder="Nom" value=<?php echo $nom ?> readonly="">
            </div>
        </div>


        <!-- Password -->
        <input type="password" id="password" class="form-control" placeholder="Password" aria-describedby="defaultRegisterFormPasswordHelpBlock">
        <small id="defaultRegisterFormPasswordHelpBlock" class="form-text text-muted mb-4">
            Au moins 8 characteres et 1 chiffre
        </small>

        <!-- Password -->
        <input type="password" id="passwordcf" class="form-control" placeholder="Password" aria-describedby="defaultRegisterFormPasswordHelpBlock">
        <small id="defaultRegisterFormPasswordHelpBlock" class="form-text text-muted mb-4">
            Confirmez votre Password
        </small>

        <!-- Sign up button -->
        <button class="btn btn-info my-4 btn-block" type="submit">Cliquez bande de salope</button>


        <hr>

        <!-- Terms of service -->
        <p>By clicking
            <em>Envoyer</em> you agree to our
            <a href="terms.jpg" target="_blank">terms of service</a>

        </form>
    </body>
</html>