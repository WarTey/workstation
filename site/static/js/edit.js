$(document).ready(function() {
    $('#customSwitch').click(function() {
        $(this).parents('form').find('input[name="old_password"]').attr("placeholder", $(this)[0].checked ? "Workstation Password" : "Old Password");
    });
});
