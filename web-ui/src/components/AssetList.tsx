'use client';

import { useState, useEffect, useCallback, useMemo } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { Package, RefreshCw, ExternalLink, AlertCircle, TrendingUp } from 'lucide-react';

interface AssetListProps {
  refreshTrigger: number;
}

interface Asset {
  mint: string;
  name: string;
  type: string;
  value: number;
  balance: number;
  description: string;
  icon: string;
}

export function AssetList({ refreshTrigger }: AssetListProps) {
  const { connected, publicKey } = useWallet();
  const [loading, setLoading] = useState(false);
  const [assets, setAssets] = useState<Asset[]>([]);

  // Example assets for demonstration (always shown)
  const exampleAssets = useMemo(() => [
    {
      mint: 'Demo1234567890abcdef1234567890abcdef12345678',
      name: 'Manhattan Penthouse Suite',
      type: 'real_estate',
      value: 4500000,
      balance: 1,
      description: 'Luxury 3BR penthouse in Manhattan with panoramic city views, premium finishes, and rooftop access.',
      icon: '🏠'
    },
    {
      mint: 'Demo2345678901bcdef23456789012cdef23456789',
      name: 'Commercial Invoice #INV-2024-001',
      type: 'invoice',
      value: 85000,
      balance: 1,
      description: 'Healthcare equipment supply invoice with Net 30 payment terms from established medical supplier.',
      icon: '📄'
    },
    {
      mint: 'Demo3456789012cdef34567890123def345678901',
      name: 'Rare Earth Minerals Portfolio',
      type: 'commodity',
      value: 320000,
      balance: 1,
      description: '500kg mixed rare earth elements including lithium, cobalt, and neodymium for battery production.',
      icon: '⚡'
    },
    {
      mint: 'Demo4567890123def45678901234ef456789012',
      name: 'Digital Art Collection #NFT-2024',
      type: 'art',
      value: 125000,
      balance: 1,
      description: 'Curated collection of 10 digital artworks by emerging artists, authenticated and verified.',
      icon: '🎨'
    },
  ], []);

  const fetchAssets = useCallback(async () => {
    if (!connected || !publicKey) {
      setAssets(exampleAssets);
      return;
    }

    setLoading(true);
    try {
      // Call the real Rust API server to get owned assets
      const response = await fetch('http://127.0.0.1:3001/api/assets', {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (!response.ok) {
        throw new Error(`API call failed: ${response.status}`);
      }

      const apiAssets = await response.json();
      
      // Transform API response to our Asset interface
      const transformedAssets = apiAssets.map((asset: any) => ({
        mint: asset.mint,
        name: asset.metadata?.name || `Token ${asset.mint.slice(0, 8)}...`,
        type: asset.metadata?.asset_type || 'unknown',
        value: asset.metadata?.value || 0,
        balance: asset.balance,
        description: asset.metadata?.description || 'No description available',
        icon: getAssetIcon(asset.metadata?.asset_type || 'unknown'),
      }));

      // If no real assets, show examples with a note
      if (transformedAssets.length === 0) {
        setAssets(exampleAssets);
      } else {
        setAssets(transformedAssets);
      }
      
    } catch (error) {
      console.error('Failed to fetch assets:', error);
      // Fallback to example data if API fails
      setAssets(exampleAssets);
    } finally {
      setLoading(false);
    }
  }, [connected, publicKey, exampleAssets]);

  const getAssetIcon = (assetType: string) => {
    const iconMap: { [key: string]: string } = {
      'real_estate': '🏠',
      'invoice': '📄',
      'commodity': '⚡',
      'security': '📈',
      'art': '🎨',
      'demo': '🏆',
      'test': '🧪',
      'unknown': '💎',
    };
    return iconMap[assetType] || '💎';
  };

  useEffect(() => {
    fetchAssets();
  }, [connected, publicKey, refreshTrigger, fetchAssets]);

  const totalValue = assets.reduce((sum, asset) => sum + asset.value * asset.balance, 0);
  const isUsingExamples = !connected || assets === exampleAssets;

  return (
    <div className="bg-white rounded-2xl shadow-sm border border-gray-200 p-8">
      <div className="flex items-center justify-between mb-8">
        <div className="flex items-center space-x-4">
          <div className="w-12 h-12 bg-gradient-to-br from-green-500 to-green-600 rounded-xl flex items-center justify-center">
            <Package className="w-6 h-6 text-white" />
          </div>
          <div>
            <h2 className="text-2xl font-bold text-gray-900">Asset Portfolio</h2>
            <p className="text-gray-600">
              {connected ? 'Your tokenized assets on Solana' : 'Example tokenized assets'}
            </p>
          </div>
        </div>
        
        <button
          onClick={fetchAssets}
          disabled={loading}
          className="flex items-center space-x-2 px-4 py-2 bg-green-600 text-white rounded-xl hover:bg-green-700 disabled:opacity-50 transition-all duration-200 shadow-sm hover:shadow-md"
        >
          <RefreshCw className={`w-4 h-4 ${loading ? 'animate-spin' : ''}`} />
          <span>Refresh</span>
        </button>
      </div>

      {!connected && (
        <div className="mb-6 p-4 bg-blue-50 border border-blue-200 rounded-xl flex items-start space-x-3">
          <AlertCircle className="w-5 h-5 text-blue-600 mt-0.5 flex-shrink-0" />
          <div>
            <h3 className="font-medium text-blue-900 mb-1">Example Portfolio View</h3>
            <p className="text-sm text-blue-700">
              Connect your wallet to see your actual tokenized assets. Below are example assets to demonstrate the interface.
            </p>
          </div>
        </div>
      )}

      {loading ? (
        <div className="text-center py-16">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-green-600 mx-auto mb-4"></div>
          <p className="text-gray-600 font-medium">Discovering your assets...</p>
        </div>
      ) : (
        <>
          {/* Portfolio Summary */}
          <div className="mb-6 p-6 bg-gradient-to-r from-green-50 to-blue-50 rounded-xl border border-gray-200">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-gray-600">
                  {isUsingExamples ? 'Example ' : ''}Portfolio Value
                </p>
                <p className="text-3xl font-bold text-gray-900">${totalValue.toLocaleString()}</p>
                <p className="text-sm text-gray-600 mt-1">
                  {assets.length} asset{assets.length !== 1 ? 's' : ''} • 
                  {assets.reduce((sum, asset) => sum + asset.balance, 0)} token{assets.reduce((sum, asset) => sum + asset.balance, 0) !== 1 ? 's' : ''}
                </p>
              </div>
              <TrendingUp className="w-12 h-12 text-green-600" />
            </div>
          </div>

          {/* Assets Grid */}
          <div className="space-y-4">
            {assets.map((asset, index) => (
              <div
                key={asset.mint}
                className="border border-gray-200 rounded-xl p-6 hover:border-green-300 hover:shadow-md transition-all duration-200 group"
              >
                <div className="flex items-start justify-between">
                  <div className="flex items-start space-x-4 flex-1">
                    <div className="text-4xl p-3 bg-gray-50 rounded-xl group-hover:bg-green-50 transition-colors">
                      {asset.icon}
                    </div>
                    <div className="flex-1">
                      <div className="flex items-start justify-between mb-3">
                        <h3 className="text-xl font-semibold text-gray-900 group-hover:text-green-700 transition-colors">
                          {asset.name}
                        </h3>
                        {!isUsingExamples && (
                          <a
                            href={`https://explorer.solana.com/address/${asset.mint}?cluster=devnet`}
                            target="_blank"
                            rel="noopener noreferrer"
                            className="flex items-center space-x-1 text-blue-600 hover:text-blue-700 text-sm opacity-0 group-hover:opacity-100 transition-opacity"
                          >
                            <span>View on Explorer</span>
                            <ExternalLink className="w-4 h-4" />
                          </a>
                        )}
                      </div>
                      
                      <p className="text-gray-600 text-sm mb-4 leading-relaxed">{asset.description}</p>
                      
                      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                        <div className="bg-gray-50 p-3 rounded-lg">
                          <span className="text-xs text-gray-500 uppercase tracking-wide">Type</span>
                          <div className="font-medium text-gray-900 capitalize">{asset.type.replace('_', ' ')}</div>
                        </div>
                        <div className="bg-gray-50 p-3 rounded-lg">
                          <span className="text-xs text-gray-500 uppercase tracking-wide">Value</span>
                          <div className="font-medium text-gray-900">${asset.value.toLocaleString()}</div>
                        </div>
                        <div className="bg-gray-50 p-3 rounded-lg">
                          <span className="text-xs text-gray-500 uppercase tracking-wide">Balance</span>
                          <div className="font-medium text-gray-900">{asset.balance} token{asset.balance !== 1 ? 's' : ''}</div>
                        </div>
                        <div className="bg-green-50 p-3 rounded-lg">
                          <span className="text-xs text-green-600 uppercase tracking-wide">Total Value</span>
                          <div className="font-semibold text-green-700">
                            ${(asset.value * asset.balance).toLocaleString()}
                          </div>
                        </div>
                      </div>
                      
                      {!isUsingExamples && (
                        <div className="mt-4 pt-4 border-t border-gray-100">
                          <span className="text-xs text-gray-500 uppercase tracking-wide">Mint Address</span>
                          <div className="font-mono text-xs text-gray-700 break-all mt-1 bg-gray-50 p-2 rounded">
                            {asset.mint}
                          </div>
                        </div>
                      )}
                    </div>
                  </div>
                </div>
              </div>
            ))}
          </div>

          {isUsingExamples && (
            <div className="mt-8 p-6 bg-gray-50 border border-gray-200 rounded-xl">
              <h4 className="font-semibold text-gray-900 mb-2 flex items-center space-x-2">
                <span>💡</span>
                <span>Ready to Get Started?</span>
              </h4>
              <p className="text-sm text-gray-700 leading-relaxed">
                Connect your Solana wallet and head to the "Tokenize Assets" tab to create your first real tokenized asset. 
                Your portfolio will appear here with live blockchain data and real transaction history.
              </p>
            </div>
          )}
        </>
      )}
    </div>
  );
} 