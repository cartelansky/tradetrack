let selectedNetwork = null;
let selectedToken = null;
let selectedPlan = null;

// Ağ adresleri - Her ağ için tek bir adres kullanılacak
const NETWORK_ADDRESSES = {
    solana: "DK6BeX2gK4sVHJtBTzmC9mKcAMqAzGbNhwF8HvyB4Bxv",  // Solana için tek adres
    polygon: "0x6374fE5F54a9B6847cB4a368cB28b9635B67c604"     // Polygon için tek adres
};

// API URL'ini environment'a göre ayarla
const API_URL = process.env.NODE_ENV === 'production' 
  ? 'https://tradetrack-backend.onrender.com/api'
  : 'http://localhost:8080/api';

// Ağ seçimi
function selectNetwork(network) {
    selectedNetwork = network;
    document.querySelectorAll('.network-option').forEach(option => {
        option.classList.remove('selected');
    });
    event.currentTarget.classList.add('selected');
    updateWalletAddress();
}

// Token seçimi
function selectToken(token) {
    selectedToken = token;
    document.querySelectorAll('.token-option').forEach(option => {
        option.classList.remove('selected');
    });
    event.currentTarget.classList.add('selected');
    updateWalletAddress();
}

// Cüzdan adresini güncelle
function updateWalletAddress() {
    const addressContainer = document.getElementById('wallet-address-container');
    const walletAddress = document.getElementById('wallet-address');
    const networkName = document.getElementById('network-name');
    const amountToSend = document.getElementById('amount-to-send');
    const verifyBtn = document.getElementById('verify-payment-btn');
    const paymentSummary = document.getElementById('payment-summary');
    
    if (selectedNetwork && selectedToken && selectedPlan) {
        // Plan seçimine göre tutarı belirle
        const amount = selectedPlan === 'Pro' ? '100' : '50';
        
        // Wallet address container'ı göster
        addressContainer.classList.remove('hidden');
        walletAddress.textContent = NETWORK_ADDRESSES[selectedNetwork];
        networkName.textContent = selectedNetwork.toUpperCase();
        amountToSend.textContent = `${amount} ${selectedToken.toUpperCase()}`;
        verifyBtn.disabled = false;
        
        // Payment summary'i güncelle
        document.getElementById('selected-network').textContent = selectedNetwork.toUpperCase();
        document.getElementById('selected-token').textContent = selectedToken.toUpperCase();
        document.getElementById('final-amount').textContent = `${amount} ${selectedToken.toUpperCase()}`;
        
        // Payment summary'i göster
        paymentSummary.classList.remove('hidden');
    }
}

// Plan seçimi fonksiyonu
function selectPlanType(plan) {
    selectedPlan = plan;
    // Plan seçimini güncelle
    document.querySelectorAll('.plan-option').forEach(option => {
        option.classList.remove('selected');
    });
    document.getElementById(`${plan.toLowerCase()}-plan`).classList.add('selected');
    
    // Eğer ağ ve token seçiliyse ödeme bilgilerini güncelle
    if (selectedNetwork && selectedToken) {
        updateWalletAddress();
    }
}

// Adresi kopyala
function copyAddress() {
    const address = document.getElementById('wallet-address').textContent;
    navigator.clipboard.writeText(address).then(() => {
        const copyBtn = document.getElementById('copy-btn');
        copyBtn.innerHTML = '<i class="fas fa-check"></i>';
        setTimeout(() => {
            copyBtn.innerHTML = '<i class="fas fa-copy"></i>';
        }, 2000);
    });
}

// Ödeme doğrulama
async function verifyPayment() {
    const txHash = document.getElementById('tx-hash').value.trim();
    const email = localStorage.getItem('userEmail'); // Kullanıcı email'ini local storage'dan al
    
    if (!txHash) {
        alert('Please enter transaction hash');
        return;
    }

    try {
        const response = await fetch(`${API_URL}/verify-payment`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                tx_hash: txHash,
                network: selectedNetwork,
                email: email
            })
        });

        const data = await response.json();
        if (data.success) {
            alert('Payment verified! Redirecting to dashboard...');
            window.location.href = '/dashboard';
        } else {
            alert(data.message || 'Payment verification failed');
        }
    } catch (error) {
        console.error('Payment verification failed:', error);
    }
}

// Doğrulama simülasyonu
async function simulateVerification() {
    return new Promise((resolve) => {
        setTimeout(resolve, 2000);
    });
}

// Sayfa yüklendiğinde seçili planı işaretle
window.addEventListener('DOMContentLoaded', () => {
    const urlParams = new URLSearchParams(window.location.search);
    const plan = urlParams.get('plan') || 'Basic';
    const amount = urlParams.get('amount') || '50';
    
    document.getElementById('selected-plan').textContent = plan;
    document.getElementById('payment-amount').textContent = amount;
}); 