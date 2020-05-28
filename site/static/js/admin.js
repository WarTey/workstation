$(document).ready(function() {
    var table = $('#table').DataTable({
        "scrollX": true,
        "scrollY": '55vh',
        "scrollCollapse": true
    });

    $(".approve, .super, .delete, .reset").submit(function(event) {
        event.preventDefault();
        var form = $(this);
        $.ajax({
            url : form.attr('action'),
            type: form.attr('method'),
            data: {
                email: form.find('input[name="email"]').val()
            },
            success: function(data) {
                noty(data.message, data.success);
                if ((form.attr('class') == "approve" || form.attr('class') == "super") && data.success)
                    form.find(':submit').removeClass(data.status ? "btn-danger fa-user-times" : "btn-success fa-user-check").addClass(data.status ? "btn-success fa-user-check" : "btn-danger fa-user-times")
                else if (form.attr('class') == "delete" && data.success)
                    table.row(form.parents('tr')).remove().draw();
                else if (form.attr('class') == "reset" && data.success)
                    form.parents('tr').find('.token').text(data.token).parents('tr').find('.activated').text("No");
                table.columns.adjust();
            }
        });
    });
});
