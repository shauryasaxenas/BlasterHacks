
fetch("products.json")
.then(function(response){
	return response.json();
})
.then(function(products){
	let placeholder = document.querySelector("#data-output");
	let out = "";
	for(let product of products){
        var month, day, year, timeHour, timeMin, timeOfDay, full_assignment;

        month = product.dueAt.slice(5,8);
        day = product.dueAt.slice(8,10);
        year = product.dueAt.slice(2,4);

        timeHour = parseInt(product.dueAt.slice(11,14))
        timeMin = product.dueAt.slice(14,16)
        timeOfDay = "AM"

        if (timeHour > 11) 
        {
            if (timeHour != 12)
            {
                timeHour = timeHour - 12;
            }
            
            timeOfDay = "PM"
        }

        full_assignment = product.course + " - " + product.name + " (" + month + day 
        + '-' + year + " @" + timeHour + ":" + timeMin + " " + timeOfDay 
        +")"



		out += `
			<tr>
				<td>${full_assignment}</td>
			</tr>
		`;
	}

	placeholder.innerHTML = out;
});