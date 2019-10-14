<?php 
$confirmPassw = "false" ;
$problPassw = "false" ;
$formAddr = "formulaire.php".$_GET['url'];
if(isset($_POST['password']) && isset($_POST['passwordcf']) && $_POST['password'] == $_POST['passwordcf'] && isset($_POST['disableForm']) && $_POST['disableForm'] == "true" ) {
    $confirmPassw = "true" ;
    $problPassw = "false" ;
}
elseif(isset($_POST['disableForm']) && $_POST['disableForm'] == "true" ) {
    $problPassw = "true" ;
    $confirmPassw = "false" ;
}
?>

<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>Formulaire Cyber</title>
        <link href="bootstrap/css/bootstrap.css" rel="stylesheet">
    </head>

    <body>
        <div class="container jumbotron"> 
        <?php
        init();
        if($enable == 1){
        ?>    

        <form class="text-center " method="post" action=<?php $formAddr ?> >

        <p class="h4 mb-4">Formulaire Cyber</p>

        <div class="form-row mb-4">
            <div class="col">
                <!-- First name -->
                <input type="text" name="prenom" id="prenom" class="form-control" placeholder="Prenom" value=<?php echo $prenom ?> readonly="">
            </div>
            <div class="col">
                <!-- Last name -->
                <input type="text" name="nom" id="nom" class="form-control" placeholder="Nom" value=<?php echo $nom ?> readonly="">
            </div>
        </div>

        <?php
        
        if ($confirmPassw == "true") {
            ?>
            <div class="alert alert-success">
                <strong>Success!</strong> Le mot de passe est bien défini
            </div>
            <?php
            modifyFile();
        }
        elseif ($problPassw == "true") {
            ?>
            <div class="alert alert-danger">
                <strong>Failure!</strong> Le mot de passe est mal défini
            </div>
            <?php 
        }
        if ($confirmPassw == "false" && $problPassw == "false")
        {
        ?>
        

        <!-- Password -->
        <input type="password" name="password" id="password" class="form-control" placeholder="Password" aria-describedby="passwordHelpBlock" required>
        <small id="passwordHelpBlock" class="form-text text-muted mb-4">
            Au moins 8 characteres et 1 chiffre
        </small>

        <!-- Password -->
        <input type="password" name="passwordcf" id="passwordcf" class="form-control" placeholder="Password" aria-describedby="passwordcfHelpBlock" required>
        <small id="passwordcfHelpBlock" class="form-text text-muted mb-4">
            Confirmez votre Password
        </small>

        <!-- Sign up button -->
        <input class="btn btn-info my-4 btn-block" type="submit" value="Envoyer" />

        <input type="hidden" id="disableForm" name="disableForm" value="true" />


        <hr>

        <!-- Terms of service -->
        <p>By clicking
            <em>Envoyer</em> you agree to our
            <a href=https://random.dog/ target="_blank">terms of service</a>
        <?php 
        }
        ?>
        </form>
        <?php 
        }
        if($enable == 0){   
            echo "Le lien n'est pas bon" ;
        }
        ?>   
        </div>
    </body>
</html>

<?php
function init()
{
    global $prenom;
    global $nom;
    global $enable;
    $ligne = 1; // compteur de ligne
    $fic1 = fopen("bdd.txt", "a+");
    while($tab=fgetcsv($fic1,1024,';'))
    {
        $ligne ++;
        
        if($tab[0] != "" && $tab[2]==$_GET['url']) {
            $prenom = $tab[0];
            $nom = $tab[1];
            $enable = 1;
        }
    }
    fclose($fic1);
}

function modifyFile() 
{
    $enable = 1;
        if($_POST['password'] == $_POST['passwordcf']) {
            $ligne = 1; // compteur de ligne
            $fic2 = fopen("addUserToAD.txt", "a+");
            while($tab=fgetcsv($fic2,1024,';'))
            {
                $ligne ++;
                
                if($tab[0] == $_POST['prenom'] && $tab[1] == $_POST['nom']) {
                    $enable = 0;
                }
            }
            if($enable == 1) {
                $psw_encod = base64_encode($_POST['password']);
                $concat = $_POST['prenom'] . ";" . $_POST['nom'] . ";" . $psw_encod ;
                $concat .= "\r\n";
                fwrite($fic2,$concat);
            }
        }
        fclose($fic2);
}
?>