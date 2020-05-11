$(function () {
	$('#password_input').popover({
		container: 'body',
		delay: {
            "show": 200
        }
	})
	
	$('#password_confirm_input').popover({
		container: 'body',
		delay: {
            "show": 200
        }
  	})
})

$("#password_input").on("click input", function(){

	var myInput = document.getElementById("password_input");
 	var letter = document.getElementById("letter");
 	var capital = document.getElementById("capital");
	var number = document.getElementById("number");
	var length = document.getElementById("length");

	passwordProgressBar();

	// When the user starts to type something inside the password field
	// Validate lowercase letters
	var lowerCaseLetters = /[a-z]/g;
	if(myInput.value.match(lowerCaseLetters)) {  
		letter.classList.remove("invalid");
		letter.classList.add("valid");
	} else {
		letter.classList.remove("valid");
		letter.classList.add("invalid");
	}
 
	// Validate capital letters
	var upperCaseLetters = /[A-Z]/g;
	if(myInput.value.match(upperCaseLetters)) {  
		capital.classList.remove("invalid");
		capital.classList.add("valid");
	} else {
		capital.classList.remove("valid");
		capital.classList.add("invalid");
	}

	// Validate numbers
	var numbers = /[0-9]/g;
	if(myInput.value.match(numbers)) {  
		number.classList.remove("invalid");
		number.classList.add("valid");
	} else {
		number.classList.remove("valid");
		number.classList.add("invalid");
	}
  
	// Validate length
	if(myInput.value.length >= 13) {
		length.classList.remove("invalid");
		length.classList.add("valid");
	} else {
		length.classList.remove("valid");
		length.classList.add("invalid");
	}

	//var specialCharacters = /[.*+\-?^${}()|[\]\\]/g
	var specialCharacters = /\W|_/g
	if(myInput.value.match(specialCharacters)){
		specialCharacter.classList.remove("invalid");
		specialCharacter.classList.add("valid");
	} else {
		specialCharacter.classList.remove("valid");
		specialCharacter.classList.add("invalid");
	}
});

function passwordProgressBar() {
	var progressBar = document.getElementById("progress");

	result = zxcvbn(document.getElementById("password_input").value, user_inputs=[])

	if(result.score > 0){
		progressBar.style.display = "block";
	} else {
		progressBar.style.display = "none";
	}

	$('.passwordHacked').css('display', 'block')
	
	$(".passwordHackedTime").text(result.crack_times_display.online_no_throttling_10_per_second);
	if(result.score == 0) {
		$('.progress-bar').css('width', 0 +'%').attr('aria-valuenow', 0);
	} else if(result.score == 1) {
		$('.progress-bar').css('width', 25 +'%').css('background-color', 'red').attr('aria-valuenow', 25).text(" ");
		$('.passwordStrength').css('color', 'red').css('font-weight', 'bold').css('display', 'block').text("Very Guessable Password");
	} else if(result.score == 2) {
		$('.progress-bar').css('width', 50 +'%').css('background-color', 'orange').attr('aria-valuenow', 50).text(" ");
		$('.passwordStrength').css('color', 'orange').css('font-weight', 'bold').text("Somewhat Guessable Password");
	} else if(result.score == 3) {
		$('.progress-bar').css('width', 75 +'%').css('background-color', '#00b300').attr('aria-valuenow', 75).text(" ");
		$('.passwordStrength').css('color', '#00b300').css('font-weight', 'bold').text("Safely Unguessable Password");
	} else if(result.score == 4) {
		$('.progress-bar').css('width', 100 +'%').css('background-color', '#1a1aff').attr('aria-valuenow', 100).text(" ");
		$('.passwordStrength').css('color', '#1a1aff').css('font-weight', 'bold').text("Very Unguessable Password");
	}
}

$("#password_confirm_input").on("click input", function(){
	var myInput = document.getElementById("password_confirm_input");
	var initialPassword = document.getElementById("password_input");
	var same = document.getElementById("same");
	var result = myInput.value.localeCompare(initialPassword.value);

	// When the user starts to type something inside the password confirmation field
	// Validate that the two passwords are equal
	if(!result) {
		same.classList.remove("invalid");
		same.classList.add("valid");
	} else {
		same.classList.remove("valid");
		same.classList.add("invalid");
	}
});
