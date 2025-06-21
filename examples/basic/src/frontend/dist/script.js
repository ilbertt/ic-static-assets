class Particle {
    constructor(x, y, vx = 0, vy = 0) {
        this.x = x;
        this.y = y;
        this.vx = vx;
        this.vy = vy;
        this.size = Math.random() * 3 + 1;
        this.life = 1.0;
        this.decay = Math.random() * 0.02 + 0.005;
        this.color = `hsl(${Math.random() * 360}, 70%, 60%)`;
        this.connections = [];
    }

    update() {
        this.x += this.vx;
        this.y += this.vy;
        
        if (gravity) {
            this.vy += 0.1;
        }
        
        this.life -= this.decay;
        
        // Bounce off walls
        if (this.x <= 0 || this.x >= canvas.width) {
            this.vx *= -0.8;
            this.x = Math.max(0, Math.min(canvas.width, this.x));
        }
        if (this.y <= 0 || this.y >= canvas.height) {
            this.vy *= -0.8;
            this.y = Math.max(0, Math.min(canvas.height, this.y));
        }
        
        // Add some randomness
        this.vx += (Math.random() - 0.5) * 0.5;
        this.vy += (Math.random() - 0.5) * 0.5;
        
        // Limit velocity
        const maxVel = 5;
        this.vx = Math.max(-maxVel, Math.min(maxVel, this.vx));
        this.vy = Math.max(-maxVel, Math.min(maxVel, this.vy));
    }

    draw(ctx) {
        ctx.save();
        ctx.globalAlpha = this.life;
        ctx.fillStyle = this.color;
        ctx.beginPath();
        ctx.arc(this.x, this.y, this.size, 0, Math.PI * 2);
        ctx.fill();
        ctx.restore();
    }
}

// Global variables
let canvas, ctx;
let particles = [];
let gravity = true;
let isPaused = false;
let lastTime = 0;
let frameCount = 0;
let fps = 0;
let colorScheme = 0;

const colorSchemes = [
    () => `hsl(${Math.random() * 360}, 70%, 60%)`,
    () => `hsl(${200 + Math.random() * 60}, 80%, 60%)`,
    () => `hsl(${Math.random() * 60}, 90%, 60%)`,
    () => `rgb(${Math.random() * 255}, ${Math.random() * 255}, ${Math.random() * 255})`
];

function resizeCanvas() {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
}

function addParticles(count) {
    for (let i = 0; i < count; i++) {
        const x = Math.random() * canvas.width;
        const y = Math.random() * canvas.height;
        const vx = (Math.random() - 0.5) * 4;
        const vy = (Math.random() - 0.5) * 4;
        const particle = new Particle(x, y, vx, vy);
        particle.color = colorSchemes[colorScheme]();
        particles.push(particle);
    }
}

function clearParticles() {
    particles = [];
}

function toggleGravity() {
    gravity = !gravity;
    document.getElementById('gravityStatus').textContent = gravity ? 'On' : 'Off';
}

function changeColorScheme() {
    colorScheme = (colorScheme + 1) % colorSchemes.length;
    particles.forEach(particle => {
        particle.color = colorSchemes[colorScheme]();
    });
}

function drawConnections() {
    const maxDistance = 100;
    
    for (let i = 0; i < particles.length; i++) {
        for (let j = i + 1; j < particles.length; j++) {
            const dx = particles[i].x - particles[j].x;
            const dy = particles[i].y - particles[j].y;
            const distance = Math.sqrt(dx * dx + dy * dy);
            
            if (distance < maxDistance) {
                const opacity = (1 - distance / maxDistance) * 0.3;
                ctx.strokeStyle = `rgba(0, 212, 255, ${opacity})`;
                ctx.lineWidth = 1;
                ctx.beginPath();
                ctx.moveTo(particles[i].x, particles[i].y);
                ctx.lineTo(particles[j].x, particles[j].y);
                ctx.stroke();
            }
        }
    }
}

function animate(currentTime) {
    if (!isPaused) {
        // Clear canvas
        ctx.fillStyle = 'rgba(12, 12, 12, 0.1)';
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        
        // Update and draw particles
        particles = particles.filter(particle => particle.life > 0);
        
        particles.forEach(particle => {
            particle.update();
            particle.draw(ctx);
        });
        
        // Draw connections
        drawConnections();
        
        // Update stats
        document.getElementById('particleCount').textContent = particles.length;
        document.getElementById('activeParticles').textContent = particles.length;
        
        // Calculate FPS
        frameCount++;
        if (currentTime - lastTime >= 1000) {
            fps = frameCount;
            frameCount = 0;
            lastTime = currentTime;
        }
        document.getElementById('fps').textContent = fps;
    }
    
    requestAnimationFrame(animate);
}

// Make functions globally available
window.addParticles = addParticles;
window.clearParticles = clearParticles;
window.toggleGravity = toggleGravity;
window.changeColorScheme = changeColorScheme;

// Initialize when DOM is ready
document.addEventListener('DOMContentLoaded', function() {
    // Canvas setup
    canvas = document.getElementById('canvas');
    ctx = canvas.getContext('2d');
    
    // Event listeners
    window.addEventListener('resize', resizeCanvas);
    
    canvas.addEventListener('click', (e) => {
        const rect = canvas.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;
        
        for (let i = 0; i < 10; i++) {
            const particle = new Particle(x, y);
            particle.color = colorSchemes[colorScheme]();
            particles.push(particle);
        }
    });
    
    canvas.addEventListener('mousemove', (e) => {
        if (e.buttons === 1) { // Left mouse button
            const rect = canvas.getBoundingClientRect();
            const x = e.clientX - rect.left;
            const y = e.clientY - rect.top;
            
            const particle = new Particle(x, y);
            particle.color = colorSchemes[colorScheme]();
            particles.push(particle);
        }
    });
    
    document.addEventListener('keydown', (e) => {
        if (e.code === 'Space') {
            e.preventDefault();
            isPaused = !isPaused;
        }
    });
    
    // Initialize
    resizeCanvas();
    addParticles(100);
    animate(0);
    
    // Add some initial particles with different behaviors
    setTimeout(() => {
        for (let i = 0; i < 50; i++) {
            const x = Math.random() * canvas.width;
            const y = Math.random() * canvas.height;
            const particle = new Particle(x, y, 0, 0);
            particle.color = colorSchemes[colorScheme]();
            particle.size = Math.random() * 5 + 2;
            particles.push(particle);
        }
    }, 1000);
}); 