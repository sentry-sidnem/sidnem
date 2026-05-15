// Interactive Architecture Tabs
const archItems = document.querySelectorAll('.arch-item');

archItems.forEach(item => {
    item.addEventListener('click', () => {
        // Reset all items
        archItems.forEach(i => i.setAttribute('data-active', 'false'));
        // Set clicked item to active
        item.setAttribute('data-active', 'true');
    });
});

// Simple Scroll Reveal
const observerOptions = {
    threshold: 0.1
};

const observer = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            entry.target.style.opacity = '1';
            entry.target.style.transform = 'translateY(0)';
            observer.unobserve(entry.target);
        }
    });
}, observerOptions);

document.querySelectorAll('.card, .arch-item, .terminal, .step-card, .value-item, .problem-item').forEach(el => {
    el.style.opacity = '0';
    el.style.transform = 'translateY(20px)';
    el.style.transition = 'all 0.6s cubic-bezier(0.4, 0, 0.2, 1)';
    observer.observe(el);
});

// Terminal Animation Simulation
const terminalBody = document.querySelector('.terminal-body');
if (terminalBody) {
    // We could add more complex terminal typing effects here if needed
}

// Navigation scroll effect
const nav = document.querySelector('.sticky-nav');
window.addEventListener('scroll', () => {
    if (window.scrollY > 50) {
        nav.style.backgroundColor = 'rgba(10, 10, 10, 0.95)';
        nav.style.borderBottomColor = 'var(--border-strong)';
    } else {
        nav.style.backgroundColor = 'rgba(10, 10, 10, 0.8)';
        nav.style.borderBottomColor = 'var(--border-subtle)';
    }
});
