$(document).ready(function() {
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

    function progressBar() {
        var result = zxcvbn($("#password").val(), user_inputs = [])
        if(result.score > 0) {
            if ($('.user_card').css('height') == '560px')
                $('.user_card').css('height', '630px');
            else if ($('.user_card').css('height') == '465px')
                $('.user_card').css('height', '535px');
            $('.progress').css('display', 'flex');
            $('#password-strength').fadeIn();
        } else {
            if ($('.user_card').css('height') == '495px' || $('.user_card').css('height') == '630px')
                $('.user_card').css('height', '560px');
            else if ($('.user_card').css('height') == '400px' || $('.user_card').css('height') == '535px')
                $('.user_card').css('height', '465px');
            $('.progress').css('display', 'none');
            $('#password-strength').css('display', 'none');
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
            if ($("#password").val().match(/[a-z]/g))
                $("#letter").removeClass("invalid").addClass("valid");
            else
                $("#letter").removeClass("valid").addClass("invalid");

            if ($("#password").val().match(/[A-Z]/g))
                $("#capital").removeClass("invalid").addClass("valid");
            else
                $("#capital").removeClass("valid").addClass("invalid");

            if ($("#password").val().match(/[0-9]/g))
                $("#number").removeClass("invalid").addClass("valid");
            else
                $("#number").removeClass("valid").addClass("invalid");

            if ($("#password").val().length >= 13)
                $("#length").removeClass("invalid").addClass("valid");
            else
                $("#length").removeClass("valid").addClass("invalid");

            if ($("#password").val().match(/\W|_/g))
                $("#special").removeClass("invalid").addClass("valid");
            else
                $("#special").removeClass("valid").addClass("invalid");

            progressBar();
        }, 200);
    });

    $("#repassword").on("click input", function() {
        setTimeout(function() {
            if ($("#password").val() == $("#repassword").val())
                $("#same").removeClass("invalid").addClass("valid");
            else
                $("#same").removeClass("valid").addClass("invalid");
        }, 200);
    });
});
