'use client';

import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { useWallet } from '@solana/wallet-adapter-react';

export function Header() {
  const { connected, publicKey } = useWallet();

  return (
    <header className="bg-white backdrop-blur-md bg-opacity-95 border-b border-gray-200/50 shadow-lg sticky top-0 z-50">
      <div className="container mx-auto px-6 py-4">
        <div className="flex items-center justify-between">
          {/* Logo and Brand */}
          <div className="flex items-center space-x-6">
            <div className="flex items-center space-x-3">
              <div className="relative">
                <div className="w-10 h-10 bg-gradient-to-br from-blue-600 via-purple-600 to-indigo-600 rounded-xl shadow-lg flex items-center justify-center">
                  <svg className="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1" />
                  </svg>
                </div>
                <div className="absolute -top-1 -right-1 w-4 h-4 bg-green-500 rounded-full border-2 border-white animate-pulse"></div>
              </div>
              <div>
                <h1 className="text-2xl font-bold bg-gradient-to-r from-gray-900 to-gray-700 bg-clip-text text-transparent">
                  Finternet SDK
                </h1>
                <p className="text-xs text-gray-500 font-medium">Tokenization Platform</p>
              </div>
            </div>
            
            {/* Status Badge */}
            <div className="hidden lg:flex items-center space-x-2 px-4 py-2 bg-gradient-to-r from-blue-50 to-indigo-50 border border-blue-200/50 rounded-full shadow-sm">
              <div className="flex items-center space-x-1">
                <div className="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
                <span className="text-sm text-blue-700 font-semibold">Live Demo</span>
              </div>
              <div className="w-1 h-4 bg-blue-200 rounded-full"></div>
              <span className="text-sm text-gray-600 font-medium">Real Blockchain</span>
            </div>
          </div>
          
          {/* Wallet Section */}
          <div className="flex items-center space-x-4">
            {/* Connection Status */}
            {connected && publicKey && (
              <div className="hidden sm:flex items-center space-x-3 px-4 py-2 bg-gradient-to-r from-green-50 to-emerald-50 border border-green-200/50 rounded-xl shadow-sm">
                <div className="flex items-center space-x-2">
                  <div className="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
                  <span className="text-sm text-green-700 font-semibold">Connected</span>
                </div>
                <div className="hidden md:block">
                  <span className="text-xs text-gray-500 font-mono">
                    {publicKey.toString().slice(0, 4)}...{publicKey.toString().slice(-4)}
                  </span>
                </div>
              </div>
            )}
            
            {/* Wallet Button */}
            <div className="relative">
              <WalletMultiButton className="!bg-gradient-to-r !from-blue-600 !to-indigo-600 hover:!from-blue-700 hover:!to-indigo-700 !rounded-xl !font-semibold !text-white !px-6 !py-3 !h-auto !shadow-lg hover:!shadow-xl !transition-all !duration-200 !border-0" />
            </div>
          </div>
        </div>
      </div>
    </header>
  );
} 