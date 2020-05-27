$(document).ready(function() {
    $('#customSwitch').click(function() {
        console.log($(this).parents('form').find('input[name="old_password"]'));
        $(this).parents('form').find('input[name="old_password"]').attr("placeholder", $(this)[0].checked ? "Workstation Password" : "Old Password");
    });
});
