'use client';

import { useState } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { Coins, Plus, Loader2, AlertCircle, CheckCircle } from 'lucide-react';

interface AssetTokenizationProps {
  onTokenCreated: () => void;
}

export function AssetTokenization({ onTokenCreated }: AssetTokenizationProps) {
  const { connected, publicKey } = useWallet();
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<{
    mint: string;
    signature: string;
  } | null>(null);

  const [formData, setFormData] = useState({
    name: '',
    description: '',
    value: '',
    assetType: 'real_estate'
  });

  const assetTypes = [
    { value: 'real_estate', label: 'Real Estate', icon: '🏠' },
    { value: 'commodity', label: 'Commodity', icon: '⚡' },
    { value: 'invoice', label: 'Invoice', icon: '📄' },
    { value: 'security', label: 'Security', icon: '📈' },
    { value: 'art', label: 'Art', icon: '🎨' },
    { value: 'other', label: 'Other', icon: '💎' },
  ];

  // Helper function to check if form is valid
  const isFormValid = () => {
    const value = parseInt(formData.value);
    return (
      formData.name.trim().length > 0 &&
      formData.description.trim().length > 0 &&
      !isNaN(value) &&
      value > 0
    );
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!connected || !publicKey) {
      alert('Please connect your wallet first using the "Select Wallet" button in the top right corner');
      return;
    }

    // Validate form data
    const value = parseInt(formData.value);
    if (isNaN(value) || value <= 0) {
      alert('Please enter a valid positive value for the asset');
      return;
    }

    if (!formData.name.trim()) {
      alert('Please enter an asset name');
      return;
    }

    if (!formData.description.trim()) {
      alert('Please enter an asset description');
      return;
    }

    setLoading(true);
    setResult(null);
    
    try {
      // Call the real Rust API server
      const response = await fetch('http://127.0.0.1:3001/api/tokenize-asset', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          name: formData.name.trim(),
          description: formData.description.trim(),
          value: value, // Now guaranteed to be positive
          asset_type: formData.assetType,
        }),
      });

      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`API call failed: ${response.status} - ${errorText}`);
      }

      const apiResult = await response.json();
      
      setResult({
        mint: apiResult.mint,
        signature: apiResult.signature,
      });
      
      // Reset form
      setFormData({
        name: '',
        description: '',
        value: '',
        assetType: 'real_estate'
      });
      
      onTokenCreated();
      
    } catch (error) {
      console.error('Token creation failed:', error);
      alert(`Token creation failed: ${error instanceof Error ? error.message : 'Unknown error'}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="bg-white rounded-2xl shadow-sm border border-gray-200 p-8">
      <div className="flex items-center space-x-4 mb-8">
        <div className="w-12 h-12 bg-gradient-to-br from-blue-500 to-blue-600 rounded-xl flex items-center justify-center">
          <Coins className="w-6 h-6 text-white" />
        </div>
        <div>
          <h2 className="text-2xl font-bold text-gray-900">Asset Tokenization</h2>
          <p className="text-gray-600">Transform real-world assets into blockchain tokens</p>
        </div>
      </div>

      {!connected && (
        <div className="mb-6 p-4 bg-blue-50 border border-blue-200 rounded-xl flex items-start space-x-3">
          <AlertCircle className="w-5 h-5 text-blue-600 mt-0.5 flex-shrink-0" />
          <div>
            <h3 className="font-medium text-blue-900 mb-1">Wallet Connection Required</h3>
            <p className="text-sm text-blue-700">
              Please connect your Solana wallet using the "Select Wallet" button in the top right corner to create real SPL tokens.
              You can still explore the form below to see what information is needed.
            </p>
          </div>
        </div>
      )}

      <form onSubmit={handleSubmit} className="space-y-6">
        <div className="grid md:grid-cols-2 gap-6">
          <div>
            <label className="block text-sm font-semibold text-gray-700 mb-2">
              Asset Name *
            </label>
            <input
              type="text"
              value={formData.name}
              onChange={(e) => setFormData(prev => ({ ...prev, name: e.target.value }))}
              className="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-colors text-gray-900 placeholder-gray-400 bg-white"
              placeholder="e.g., Manhattan Luxury Apartment #42"
              required
            />
            <p className="text-xs text-gray-500 mt-1">Give your asset a descriptive name</p>
          </div>

          <div>
            <label className="block text-sm font-semibold text-gray-700 mb-2">
              Asset Value (USD) *
            </label>
            <input
              type="number"
              min="1"
              step="1"
              value={formData.value}
              onChange={(e) => {
                const value = e.target.value;
                // Only allow positive numbers
                if (value === '' || (parseInt(value) > 0 && !isNaN(parseInt(value)))) {
                  setFormData(prev => ({ ...prev, value }));
                }
              }}
              className="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-colors text-gray-900 placeholder-gray-400 bg-white"
              placeholder="e.g., 2500000"
              required
            />
            <p className="text-xs text-gray-500 mt-1">Current market value in USD (must be positive)</p>
          </div>
        </div>

        <div>
          <label className="block text-sm font-semibold text-gray-700 mb-3">
            Asset Type *
          </label>
          <div className="grid grid-cols-2 md:grid-cols-3 gap-3">
            {assetTypes.map((type) => (
              <label
                key={type.value}
                className={`flex items-center space-x-3 p-4 border-2 rounded-xl cursor-pointer transition-all hover:border-blue-300 hover:shadow-sm ${
                  formData.assetType === type.value
                    ? 'border-blue-500 bg-blue-50 ring-2 ring-blue-200 shadow-md'
                    : 'border-gray-200 hover:border-gray-300 bg-white'
                }`}
              >
                <input
                  type="radio"
                  value={type.value}
                  checked={formData.assetType === type.value}
                  onChange={(e) => setFormData(prev => ({ ...prev, assetType: e.target.value }))}
                  className="sr-only"
                />
                <span className="text-2xl">{type.icon}</span>
                <span className={`text-sm font-medium ${
                  formData.assetType === type.value ? 'text-blue-900' : 'text-gray-900'
                }`}>
                  {type.label}
                </span>
                {formData.assetType === type.value && (
                  <div className="ml-auto">
                    <svg className="w-5 h-5 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                      <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd" />
                    </svg>
                  </div>
                )}
              </label>
            ))}
          </div>
        </div>

        <div>
          <label className="block text-sm font-semibold text-gray-700 mb-2">
            Description *
          </label>
          <textarea
            value={formData.description}
            onChange={(e) => setFormData(prev => ({ ...prev, description: e.target.value }))}
            className="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-colors text-gray-900 placeholder-gray-400 bg-white resize-none"
            rows={4}
            placeholder="Provide detailed information about your asset including location, specifications, legal details, condition, etc..."
            required
          />
          <p className="text-xs text-gray-500 mt-1">Detailed description helps with asset verification and trading</p>
        </div>

        <button
          type="submit"
          disabled={loading || !connected || !isFormValid()}
          className={`w-full flex items-center justify-center space-x-3 px-6 py-4 rounded-xl font-semibold text-white transition-all duration-200 ${
            !connected 
              ? 'bg-gray-400 cursor-not-allowed'
              : !isFormValid()
              ? 'bg-gray-400 cursor-not-allowed'
              : loading
              ? 'bg-blue-400 cursor-not-allowed'
              : 'bg-gradient-to-r from-blue-600 to-blue-700 hover:from-blue-700 hover:to-blue-800 shadow-lg hover:shadow-xl'
          }`}
        >
          {loading ? (
            <>
              <Loader2 className="w-5 h-5 animate-spin" />
              <span>Creating Token (this takes ~20 seconds)...</span>
            </>
          ) : !connected ? (
            <>
              <AlertCircle className="w-5 h-5" />
              <span>Connect Wallet to Create Token</span>
            </>
          ) : !isFormValid() ? (
            <>
              <AlertCircle className="w-5 h-5" />
              <span>Complete All Fields to Continue</span>
            </>
          ) : (
            <>
              <Plus className="w-5 h-5" />
              <span>Create SPL Token on Solana</span>
            </>
          )}
        </button>
      </form>

      {result && (
        <div className="mt-8 p-6 bg-green-50 border border-green-200 rounded-xl">
          <div className="flex items-center space-x-3 mb-4">
            <CheckCircle className="w-6 h-6 text-green-600" />
            <h3 className="text-lg font-semibold text-green-800">🎉 Token Created Successfully!</h3>
          </div>
          <div className="space-y-3">
            <div>
              <span className="text-sm font-medium text-green-700">SPL Token Mint Address:</span>
              <div className="font-mono text-sm text-green-600 break-all bg-white p-2 rounded border mt-1">
                {result.mint}
              </div>
            </div>
            <div>
              <span className="text-sm font-medium text-green-700">Blockchain Transaction:</span>
              <div className="flex items-center space-x-2 mt-1">
                <div className="font-mono text-sm text-green-600 break-all bg-white p-2 rounded border flex-1">
                  {result.signature}
                </div>
                <a
                  href={`https://explorer.solana.com/tx/${result.signature}?cluster=devnet`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="px-3 py-2 bg-green-600 text-white text-sm rounded hover:bg-green-700 transition-colors whitespace-nowrap"
                >
                  View on Explorer
                </a>
              </div>
            </div>
          </div>
        </div>
      )}

      <div className="mt-8 p-6 bg-gray-50 border border-gray-200 rounded-xl">
        <h4 className="font-semibold text-gray-900 mb-2 flex items-center space-x-2">
          <span>🔧</span>
          <span>Real Blockchain Integration</span>
        </h4>
        <p className="text-sm text-gray-700 leading-relaxed">
          This interface creates actual SPL tokens on Solana blockchain using your connected wallet. 
          Each token includes rich metadata and is immediately tradeable. The process takes about 20 seconds 
          to ensure proper blockchain confirmation.
        </p>
      </div>
    </div>
  );
} 