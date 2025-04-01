## Inspiration
Our team recognized that canvas, a tool that students use every single day, is clunky and slow. This insired us to improve upon canvas to present the information that students actually need from canvas in a manner that is user-friendly and blazingly fast.

## What it does
Our solution wraps the core functionality of canvas, assignments and grades, and presents it to the user in a single interface with no loading or clunky UI. Our solution also aggregates canvas data to provide users with AI-powered assignment summaries and a study plan for their convienience.

## How we built it
At its core, our solution is a wrapper around the canvas API with enhanced features and novel interfaces. The team used Rust to query the users canvas and organize the retrieved data. Rust also itegrates with groq's platform to provide analysis on each assignment and the student's workload as a whole.

The terminal user interface was also written in Rust with a keyboard-centric and speed emphasis allowing for technical users to navigate their assignments in a blazingly fast manner. While most of the data is presented to you in app, the interface also provides links back to relevant course resources and the canvas page itself to make submissions.The team also created a web iterface to make the took accessable to non-technical users. The web interface has the same data as the terminal interface.

## Challenges we ran into
- The canvas API is really awful and self-referencing
- Written in Rust so there was a lot of fighting the compiler
- Nobody in our group had significant web experience so we had to learn as we went when creating the web interface
- Rust async concurrency is hard


To run the TUI
