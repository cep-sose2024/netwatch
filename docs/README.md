# [Documentation](https://cep-sose2024.github.io/netwatch/posts/getting_started/)

**Welcome to the official documentation for the NetWatch project!** 

We are using [Jekyll](https://jekyllrb.com/) with the [Chirpy theme](https://github.com/cotes2020/jekyll-theme-chirpy), and host the website using GitHub Pages. This setup ensures that our documentation is not only comprehensive and easy to navigate but also visually appealing and accessible. 
## Purpose and Content
The project documentation provides comprehensive technical guidance, including detailed descriptions of the architecture, error handling, functionalities, and other crucial information essential for understanding the workflow. Additionally, we utilize `rustdoc` to document our code, ensuring that the information is well-structured and accessible for developers needing to understand or contribute to the codebase.

## Guidelines
### Language and Clarity
- **Plain Language**: Use straightforward and clear language to simplify technical complexity.
- **Concise Sentences**: Avoid long and complicated sentences to enhance readability.

### Structure and Formatting
- **Consistent Headings**: Use clear and consistent headings to structure the document effectively.
- **Tables**: Employ tables to neatly summarize data and provide comparisons.
- **Code Blocks**: Distinguish commands and code examples from the narrative text using Markdown code blocks.

### Visualization
- **Screenshots, Diagrams, and Graphics**: Integrate visual aids like screenshots, diagrams, and graphics to help explain complex information and improve comprehension.

## Contributing to the Documentation

### Install Dependencies

Install `ruby`

````
sudo apt update
sudo apt install ruby-full build-essential zlib1g-dev git
````

Install Jekyll `bundler`

````
gem install jekyll bundler
````

````
git clone https://github.com/cep-sose2024/netwatch.git
`````

Install your dependencies

````
cd netwatch
bundle
`````

### Using Jekyll locally 

````
bundle exec jekyll s
`````
