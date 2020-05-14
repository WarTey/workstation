$(function () {
    $('#password').popover({
		container: 'body',
		delay: {
            "show": 200
        }
	});

	$('#repassword').popover({
		container: 'body',
		delay: {
            "show": 200
        }
  	});
});

$("#password").on("click input", function() {
    setTimeout(function(){
        var myInput = document.getElementById("password");
        var letter = document.getElementById("letter");
        var capital = document.getElementById("capital");
        var number = document.getElementById("number");
        var length = document.getElementById("length");
        var special = document.getElementById("special");

        var lowerCaseLetters = /[a-z]/g;
        if (myInput.value.match(lowerCaseLetters)) {  
            letter.classList.remove("invalid");
            letter.classList.add("valid");
        } else {
            letter.classList.remove("valid");
            letter.classList.add("invalid");
        }

        var upperCaseLetters = /[A-Z]/g;
        if (myInput.value.match(upperCaseLetters)) {
            capital.classList.remove("invalid");
            capital.classList.add("valid");
        } else {
            capital.classList.remove("valid");
            capital.classList.add("invalid");
        }

        var numbers = /[0-9]/g;
        if (myInput.value.match(numbers)) {
            number.classList.remove("invalid");
            number.classList.add("valid");
        } else {
            number.classList.remove("valid");
            number.classList.add("invalid");
        }

        if (myInput.value.length >= 13) {
            length.classList.remove("invalid");
            length.classList.add("valid");
        } else {
            length.classList.remove("valid");
            length.classList.add("invalid");
        }

        var specialCharacters = /\W|_/g;
        if (myInput.value.match(specialCharacters)) {
            special.classList.remove("invalid");
            special.classList.add("valid");
        } else {
            special.classList.remove("valid");
            special.classList.add("invalid");
        }
    }, 200);
});

$("#repassword").on("click input", function() {
    var myInput = document.getElementById("repassword");
    var initialPassword = document.getElementById("password");
    var same = document.getElementById("same");
    var result = myInput.value.localeCompare(initialPassword.value);

    if (!result) {
        same.classList.remove("invalid");
        same.classList.add("valid");
    } else {
        same.classList.remove("valid");
        same.classList.add("invalid");
    }
});
