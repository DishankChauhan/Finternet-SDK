'use client';

import { useState } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { PublicKey } from '@solana/web3.js';
import { Send, DollarSign, Loader2, ArrowRight } from 'lucide-react';

export function PaymentInterface() {
  const { connected, publicKey } = useWallet();
  const [loading, setLoading] = useState(false);
  const [paymentType, setPaymentType] = useState<'usdc' | 'spl'>('usdc');
  const [formData, setFormData] = useState({
    to: '',
    amount: '',
    memo: '',
    tokenMint: ''
  });
  const [result, setResult] = useState<{
    signature: string;
    amount: string;
    to: string;
  } | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!connected || !publicKey) {
      alert('Please connect your wallet first');
      return;
    }

    // Basic validation
    try {
      new PublicKey(formData.to);
    } catch {
      alert('Invalid recipient address');
      return;
    }

    setLoading(true);
    try {
      // Call the real Rust API server
      const response = await fetch('http://127.0.0.1:3001/api/send-payment', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          to: formData.to,
          amount: parseFloat(formData.amount),
          memo: formData.memo || null,
          token_mint: paymentType === 'spl' ? formData.tokenMint : null,
        }),
      });

      if (!response.ok) {
        throw new Error(`Payment failed: ${response.status}`);
      }

      const apiResult = await response.json();
      
      setResult({
        signature: apiResult.signature,
        amount: formData.amount,
        to: formData.to
      });
      
      // Reset form
      setFormData({
        to: '',
        amount: '',
        memo: '',
        tokenMint: ''
      });
      
    } catch (error) {
      console.error('Payment failed:', error);
      alert(`Payment failed: ${error instanceof Error ? error.message : 'Unknown error'}`);
    } finally {
      setLoading(false);
    }
  };

  if (!connected) {
    return (
      <div className="bg-white rounded-xl shadow-lg border border-gray-100 p-8">
        <div className="text-center">
          <Send className="w-16 h-16 text-gray-400 mx-auto mb-4" />
          <h3 className="text-xl font-semibold text-gray-900 mb-2">Connect Wallet to Send Payments</h3>
          <p className="text-gray-600">
            Connect your Solana wallet to send USDC and SPL token payments globally
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="bg-white rounded-xl shadow-lg border border-gray-100 p-8">
      <div className="flex items-center space-x-3 mb-6">
        <div className="w-10 h-10 bg-green-100 rounded-lg flex items-center justify-center">
          <Send className="w-5 h-5 text-green-600" />
        </div>
        <div>
          <h2 className="text-2xl font-bold text-gray-900">Cross-Border Payments</h2>
          <p className="text-gray-600">Send USDC and SPL tokens globally</p>
        </div>
      </div>

      {/* Payment Type Selector */}
      <div className="mb-6">
        <div className="flex space-x-4">
          <button
            onClick={() => setPaymentType('usdc')}
            className={`flex items-center space-x-2 px-4 py-3 rounded-lg border transition-colors ${
              paymentType === 'usdc'
                ? 'border-blue-500 bg-blue-50 text-blue-700'
                : 'border-gray-300 hover:border-gray-400'
            }`}
          >
            <DollarSign className="w-5 h-5" />
            <span className="font-medium">USDC Payment</span>
          </button>
          <button
            onClick={() => setPaymentType('spl')}
            className={`flex items-center space-x-2 px-4 py-3 rounded-lg border transition-colors ${
              paymentType === 'spl'
                ? 'border-blue-500 bg-blue-50 text-blue-700'
                : 'border-gray-300 hover:border-gray-400'
            }`}
          >
            <div className="w-5 h-5 bg-gradient-to-r from-purple-500 to-pink-500 rounded"></div>
            <span className="font-medium">SPL Token</span>
          </button>
        </div>
      </div>

      <form onSubmit={handleSubmit} className="space-y-6">
        <div className="grid md:grid-cols-2 gap-6">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Recipient Address
            </label>
            <input
              type="text"
              value={formData.to}
              onChange={(e) => setFormData(prev => ({ ...prev, to: e.target.value }))}
              className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent font-mono text-sm text-gray-900 placeholder-gray-400 bg-white"
              placeholder="7MGLuj6eQtx8RAc1QNywvfarMX378A6Ccnbp7NVVXozR"
              required
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Amount {paymentType === 'usdc' ? '(USDC)' : '(Tokens)'}
            </label>
            <input
              type="number"
              step="0.01"
              value={formData.amount}
              onChange={(e) => setFormData(prev => ({ ...prev, amount: e.target.value }))}
              className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent text-gray-900 placeholder-gray-400 bg-white"
              placeholder={paymentType === 'usdc' ? '100.00' : '1'}
              required
            />
          </div>
        </div>

        {paymentType === 'spl' && (
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Token Mint Address
            </label>
            <input
              type="text"
              value={formData.tokenMint}
              onChange={(e) => setFormData(prev => ({ ...prev, tokenMint: e.target.value }))}
              className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent font-mono text-sm text-gray-900 placeholder-gray-400 bg-white"
              placeholder="4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU"
              required
            />
            <p className="mt-1 text-sm text-gray-500">
              Enter the mint address of the SPL token you want to send
            </p>
          </div>
        )}

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Memo (Optional)
          </label>
          <input
            type="text"
            value={formData.memo}
            onChange={(e) => setFormData(prev => ({ ...prev, memo: e.target.value }))}
            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent text-gray-900 placeholder-gray-400 bg-white"
            placeholder="Invoice payment for goods shipped..."
            maxLength={100}
          />
          <p className="mt-1 text-sm text-gray-500">
            Add a note for record keeping (max 100 characters)
          </p>
        </div>

        <button
          type="submit"
          disabled={loading}
          className="w-full flex items-center justify-center space-x-2 bg-green-600 text-white px-6 py-3 rounded-lg hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
        >
          {loading ? (
            <>
              <Loader2 className="w-5 h-5 animate-spin" />
              <span>Processing Payment...</span>
            </>
          ) : (
            <>
              <Send className="w-5 h-5" />
              <span>Send {paymentType === 'usdc' ? 'USDC' : 'Tokens'}</span>
              <ArrowRight className="w-4 h-4" />
            </>
          )}
        </button>
      </form>

      {result && (
        <div className="mt-6 p-4 bg-green-50 border border-green-200 rounded-lg">
          <h3 className="text-lg font-semibold text-green-800 mb-2">💸 Payment Sent Successfully!</h3>
          <div className="space-y-2 text-sm">
            <div>
              <span className="font-medium text-green-700">Amount:</span>
              <span className="ml-2">{result.amount} {paymentType === 'usdc' ? 'USDC' : 'tokens'}</span>
            </div>
            <div>
              <span className="font-medium text-green-700">Recipient:</span>
              <div className="font-mono text-green-600 break-all">{result.to}</div>
            </div>
            <div>
              <span className="font-medium text-green-700">Transaction:</span>
              <a
                href={`https://explorer.solana.com/tx/${result.signature}?cluster=devnet`}
                target="_blank"
                rel="noopener noreferrer"
                className="font-mono text-blue-600 hover:underline break-all block"
              >
                {result.signature}
              </a>
            </div>
          </div>
        </div>
      )}

      {/* Feature highlights */}
      <div className="mt-8 grid md:grid-cols-2 gap-4">
        <div className="p-4 bg-blue-50 border border-blue-200 rounded-lg">
          <h4 className="font-medium text-blue-800 mb-1">⚡ Instant Settlement</h4>
          <p className="text-sm text-blue-700">
            Payments settle on Solana in seconds, not days. Perfect for international trade and remittances.
          </p>
        </div>
        <div className="p-4 bg-purple-50 border border-purple-200 rounded-lg">
          <h4 className="font-medium text-purple-800 mb-1">🔍 Full Auditability</h4>
          <p className="text-sm text-purple-700">
            Every transaction is recorded on-chain with memos for compliance and accounting purposes.
          </p>
        </div>
      </div>

      <div className="mt-6 p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
        <h4 className="font-medium text-yellow-800 mb-1">🔧 Real Backend Integration</h4>
        <p className="text-sm text-yellow-700">
          This UI now calls the actual Rust Finternet SDK via HTTP API (running on :3001). 
          Real USDC and SPL token transfers are executed on Solana blockchain.
        </p>
      </div>
    </div>
  );
} 