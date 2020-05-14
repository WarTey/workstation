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

function passwordProgressBar() {
	var result = zxcvbn($("#password").val(), user_inputs = [])
    if(result.score > 0) {
        $('.user_card').css('height', '575px');
        $('.progress').css('display', 'flex');
        $('#password-strength').fadeIn();
    } else {
        $('.user_card').css('height', '510px');
        $('.progress').fadeOut();
        $('#password-strength').fadeOut();
    }

    $('#password-hacked').fadeIn();
    $("#password-time").text(result.crack_times_display.online_no_throttling_10_per_second);

	if(result.score == 0)
		$('.progress-bar').css('width', 0 + '%').attr('aria-valuenow', 0);
	else if(result.score == 1) {
		$('.progress-bar').css('width', 25 + '%').css('background-color', 'red').attr('aria-valuenow', 25).text(" ");
		$('#password-strength').css('color', 'red').css('font-weight', 'bold').css('display', 'block').text("Very Guessable Password");
	} else if(result.score == 2) {
		$('.progress-bar').css('width', 50 + '%').css('background-color', 'orange').attr('aria-valuenow', 50).text(" ");
		$('#password-strength').css('color', 'orange').css('font-weight', 'bold').text("Somewhat Guessable Password");
	} else if(result.score == 3) {
		$('.progress-bar').css('width', 75 + '%').css('background-color', '#00b300').attr('aria-valuenow', 75).text(" ");
		$('#password-strength').css('color', '#00b300').css('font-weight', 'bold').text("Safely Unguessable Password");
	} else {
		$('.progress-bar').css('width', 100 + '%').css('background-color', '#1a1aff').attr('aria-valuenow', 100).text(" ");
		$('#password-strength').css('color', '#1a1aff').css('font-weight', 'bold').text("Very Unguessable Password");
	}
}

$("#password").on("click input", function() {
    setTimeout(function() {
        var myInput = document.getElementById("password");
        var letter = document.getElementById("letter");
        var capital = document.getElementById("capital");
        var number = document.getElementById("number");
        var length = document.getElementById("length");
        var special = document.getElementById("special");

        passwordProgressBar();

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
    setTimeout(function() {
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
    }, 200);
});
