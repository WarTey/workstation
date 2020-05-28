$(document).ready(function() {
    $("#login").submit(function(event) {
        event.preventDefault();
        var form = $(this);
        $.ajax({
            url : form.attr('action'),
            type: form.attr('method'),
            data: {
                email: form.find('input[name="email"]').val(),
                password: form.find('input[name="password"]').val(),
            },
            success: function(data) {
                noty(data.message, data.success);
                if (data.success)
                    location.reload();
            }
        });
    });
});
