// Sayfa yüklendiğinde çalışacak event listener'lar
document.addEventListener('DOMContentLoaded', function() {
    // Header'daki butonlar için event listener'lar
    document.querySelector('.login-btn').addEventListener('click', showLoginModal);
    document.querySelector('.signup-btn').addEventListener('click', showSignupModal);
    
    // Get Started ve diğer signup butonları için event listener'lar
    document.querySelectorAll('[onclick="window.location.href=\'#signup\'"]').forEach(btn => {
        btn.onclick = function(e) {
            e.preventDefault();
            showSignupModal();
        }
    });

    // Modal kapatma butonları için event listener'lar
    document.querySelectorAll('.close-btn').forEach(btn => {
        btn.onclick = function() {
            this.closest('.modal').style.display = 'none';
        }
    });

    // Modal dışına tıklandığında kapatma
    window.onclick = function(event) {
        if (event.target.classList.contains('modal')) {
            event.target.style.display = 'none';
        }
    }
});

// Modal işlemleri
function showSignupModal() {
    document.getElementById('login-modal').style.display = 'none';
    document.getElementById('signup-modal').style.display = 'block';
}

function showLoginModal() {
    document.getElementById('signup-modal').style.display = 'none';
    document.getElementById('login-modal').style.display = 'block';
}

// Form işlemleri
async function handleSignup(event) {
    event.preventDefault();
    const email = document.getElementById('email').value;
    const password = document.getElementById('password').value;

    try {
        // Burada API'ye kayıt isteği atılacak
        // Başarılı kayıt sonrası:
        alert('Please check your email to verify your account.');
        document.getElementById('signup-modal').style.display = 'none';
        // Email doğrulaması simülasyonu
        setTimeout(() => {
            // URL'den seçilen planı al
            const selectedPlan = localStorage.getItem('selectedPlan') || 'Basic';
            const amount = selectedPlan === 'Pro' ? '100' : '50';
            window.location.href = `/payment.html?plan=${selectedPlan}&amount=${amount}`;
        }, 2000);
    } catch (error) {
        alert('An error occurred during signup. Please try again.');
    }
}

async function handleLogin(event) {
    event.preventDefault();
    const email = document.getElementById('login-email').value;
    const password = document.getElementById('login-password').value;

    try {
        // Burada API'ye giriş isteği atılacak
        // Başarılı giriş sonrası:
        window.location.href = '/dashboard';
    } catch (error) {
        alert('Invalid email or password.');
    }
}

// Plan seçimi için yeni fonksiyon
function selectPlan(plan) {
    localStorage.setItem('selectedPlan', plan);
    showSignupModal();
} 