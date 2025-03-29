console.log("Script is working")

fetch("products.json")
.then(function(response){
    return response.json();
})
.then(function(products){
    let placeholder = document.querySelector("#data-output");
    let out = "";
    
    for (let product of products) {
        var month, day, year, timeHour, timeMin, timeOfDay, full_assignment;

        month = product.dueAt.slice(5,8);
        day = product.dueAt.slice(8,10);
        year = product.dueAt.slice(2,4);

        timeHour = parseInt(product.dueAt.slice(11,14));
        timeMin = product.dueAt.slice(14,16);
        timeOfDay = "AM";

        if (timeHour > 11) {
            if (timeHour !== 12) {
                timeHour = timeHour - 12;
            }
            timeOfDay = "PM";
        }

        full_assignment = product.course + " - " + product.name + " (" + month + day 
        + '-' + year + " @" + timeHour + ":" + timeMin + " " + timeOfDay + ")";

        const additionalInfo = `
            <div class="additional-info">
                <p>Description: ${product.description || "No description available"}</p>
            </div>
        `;

        out += `
            <tr>
                <td>
                    <button class="assignment-toggle">
                        ${full_assignment}
                    </button>
                    <div class="additional-details" style="display: none;">
                        ${additionalInfo}
                    </div>
                </td>
            </tr>
        `;
    }

    placeholder.innerHTML = out;

    // Now, add event listeners to the buttons after the HTML is loaded
    const buttons = document.querySelectorAll('.assignment-toggle');
    buttons.forEach(button => {
        button.addEventListener('click', function() {
            const details = button.nextElementSibling; // This gets the .additional-details div
            const isVisible = details.style.display === "block";
            details.style.display = isVisible ? "none" : "block"; // Toggle visibility
        });
    });
});

fetch(grades.json)
.then(function(response){
    return response.json()
})
.then(function(elements){
    let placeholder = document.querySelector("#data-output");
    let out = "";
    
    for (let element of elements) {

        out += `
            
        `;
    }

    placeholder.innerHTML = out;

    
});