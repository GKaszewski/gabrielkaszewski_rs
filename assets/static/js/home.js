document.addEventListener('DOMContentLoaded', function () {
  const sections = document.querySelectorAll('.reveal-on-scroll');
  console.log(sections);

  const observer = new IntersectionObserver(
    (entries, observer) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          const paragraphs = entry.target.querySelectorAll('p');
          console.log(paragraphs);
          paragraphs.forEach((paragraph) => {
            paragraph.classList.add('my-animate');
            // paragraph.classList.remove('hidden-for-animation');
          });
          observer.unobserve(entry.target); // Stop observing after animation triggers
        }
      });
    },
    { threshold: 0.1 }
  );

  sections.forEach((section) => observer.observe(section));
});
