const navbar = document.getElementById('navbar');

const navbarStyles =
  'fixed z-20 flex flex-col items-center justify-center w-full p-4 bg-transparent md:flex-row';
const navbarStylesScrolled =
  'fixed flex flex-col md:flex-row w-full justify-center items-center p-4 z-20 bg-gray-900/80 backdrop-blur-md';

const handleScroll = () => {
  if (window.scrollY > 10) {
    navbar.className = navbarStylesScrolled;
  } else {
    navbar.className = navbarStyles;
  }
};

window.addEventListener('scroll', handleScroll);
window.addEventListener('load', () => {
  navbar.className = navbarStyles;
});
