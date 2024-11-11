document.addEventListener('DOMContentLoaded', () => {
  const carousels = document.querySelectorAll('[id^="carousel-"]');

  carousels.forEach((carousel) => {
    let buttons = carousel.querySelectorAll('.carousel-button');
    let activeSlide = 0;

    if (buttons.length === 1) {
      buttons[0].classList.add('opacity-0');
    }

    buttons.forEach((button, index) => {
      button.addEventListener('click', () => {
        let currentSlide = carousel.querySelector(
          '.carousel-item:not(.opacity-0)'
        );
        if (currentSlide) {
          currentSlide.classList.add('opacity-0');
        }
        let newSlide = carousel.querySelectorAll('.carousel-item')[index];
        if (newSlide) {
          newSlide.classList.remove('opacity-0');
        }
        activeSlide = index;
      });
    });

    setInterval(() => {
      if (buttons.length === 0 || buttons.length === 1) {
        return;
      }
      let nextSlide = (activeSlide + 1) % buttons.length;
      buttons[nextSlide].click();
    }, 3000);
  });
});
