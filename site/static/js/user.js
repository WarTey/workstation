$(document).ready(function() {
    $("#create-user, #create-password, #edit, #send").submit(function(event) {
        event.preventDefault();
        var form = $(this);
        var data = form.attr('id') == "create-user" ? {
            email: form.find('input[name="email"]').val(),
            lastname: form.find('input[name="lastname"]').val(),
            firstname: form.find('input[name="firstname"]').val()
        } : form.attr('id') == "create-password" ? {
            email: form.find('input[name="email"]').val(),
            password: form.find('input[name="password"]').val(),
            repassword: form.find('input[name="repassword"]').val()
        } : form.attr('id') == "edit" ? {
            email: form.find('input[name="email"]').val(),
            old_password: form.find('input[name="old_password"]').val(),
            password: form.find('input[name="password"]').val(),
            repassword: form.find('input[name="repassword"]').val()
        } : {
            email: form.find('input[name="email"]').val(),
        };
        console.log(data);
        $.ajax({
            url : form.attr('action'),
            type: form.attr('method'),
            data: data,
            success: function(data) {
                noty(data.message, data.success);
                if (form.attr('id') == "create-password" && data.success)
                    setTimeout(function() { location.reload(); }, 3500);
            }
        });
    });
});
