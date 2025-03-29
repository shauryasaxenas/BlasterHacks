console.log("Script is working");

fetch("data.json")
.then(function(response) {
    return response.json();
})
.then(function(products) {
    let placeholderAssignment = document.querySelector("#assignment-output");
    let placeholderGrades = document.querySelector("#grade-output");
    let outAssignment = "", outGrade = "";

    // Loop through products for assignments
    products.assignments.forEach(assignment => {
        var month, day, year, timeHour, timeMin, timeOfDay, full_assignment;

        // Parse the date
        month = assignment.date.slice(5, 7);
        day = assignment.date.slice(8, 10);
        year = assignment.date.slice(2, 4);

        timeHour = parseInt(assignment.date.slice(11, 13));
        timeMin = assignment.date.slice(14, 16);
        timeOfDay = "AM";

        // Convert to 12-hour time format
        if (timeHour > 11) {
            if (timeHour !== 12) {
                timeHour = timeHour - 12;
            }
            timeOfDay = "PM";
        }

        // Full assignment name
        full_assignment = assignment.course + " - " + assignment.name;

        // Due date formatted
        const dueDate = month + '/' + day + '/' + year + " @ " + timeHour + ":" + timeMin + " " + timeOfDay;

        // Additional assignment info
        const additionalInfo = `
            <div class="additional-info">
                <p>Description: ${assignment.description || "No description available"}</p>
                <p>
                    <a href="${assignment.html_url}" target="_blank">Canvas Link</a>
                </p>
            </div>
        `;

        // Build the output for the assignment row
        outAssignment += `
            <tr class="assignment-row">
                <td>
                    <button class="assignment-toggle">
                        <span style="display: inline-block; text-align: left; width: 76%;">${full_assignment}</span>
                        <span style="display: inline-block; text-align: right;">${dueDate}</span>
                    </button>
            
                    <div class="additional-details" style="display: none;">
                        ${additionalInfo}
                    </div>
                </td>
            </tr>
        `;
    });

    products.grades.forEach(grade => {
        var courseGrade = grade.course + ": " + `\n` + grade.grade + '%';
        var courseLink = `https://elearning.mines.edu/courses/${grade.course_id}/grades`;
        outGrade += `
            <tr>
                <td>
                        <a href="${courseLink}" target="_blank">${courseGrade}</a>
                </td>
            </tr>
        `;
    });

    // Insert the assignment rows into the placeholder
    placeholderAssignment.innerHTML = outAssignment;
    placeholderGrades.innerHTML = outGrade;

    // Add event listeners to toggle additional assignment details
    const buttons = document.querySelectorAll('.assignment-toggle');
    buttons.forEach(button => {
        button.addEventListener('click', function() {
            const details = button.nextElementSibling; // This gets the .additional-details div
            const row = button.closest('tr'); // Get the parent row of the button
            const isVisible = details.style.display === "block";

            // Toggle the visibility of the description
            details.style.display = isVisible ? "none" : "block"; 

            // Toggle the expanded class to adjust row height
            row.classList.toggle('expanded');
        });
    });
});
