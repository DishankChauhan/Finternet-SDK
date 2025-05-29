'use client';

import { useState } from 'react';
import { WalletConnectionProvider } from '@/components/WalletConnectionProvider';
import { Header } from '@/components/Header';
import { AssetTokenization } from '@/components/AssetTokenization';
import { AssetList } from '@/components/AssetList';
import { PaymentInterface } from '@/components/PaymentInterface';
import { WalletInfo } from '@/components/WalletInfo';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';

export default function Home() {
  const [refreshTrigger, setRefreshTrigger] = useState(0);

  const handleRefresh = () => {
    setRefreshTrigger(prev => prev + 1);
  };

  return (
    <WalletConnectionProvider>
      <div className="min-h-screen bg-gradient-to-br from-slate-50 via-blue-50 to-indigo-100">
        <Header />
        
        <main className="container mx-auto px-4 py-8">
          {/* Hero Section */}
          <div className="text-center mb-12">
            <div className="inline-flex items-center space-x-3 mb-6">
              <div className="w-12 h-12 bg-gradient-to-br from-blue-600 to-purple-600 rounded-xl flex items-center justify-center">
                <svg className="w-7 h-7 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1" />
                </svg>
              </div>
              <h1 className="text-5xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                Finternet SDK
              </h1>
            </div>
            <p className="text-2xl text-gray-700 mb-4 font-semibold">
              The Complete Tokenization & Payment Platform
            </p>
            <p className="text-lg text-gray-600 max-w-3xl mx-auto mb-8">
              Transform any asset into blockchain tokens, send global payments, and manage your portfolio
              with enterprise-grade infrastructure built on Solana.
            </p>
            
            {/* Feature highlights */}
            <div className="flex flex-wrap justify-center gap-4 mb-8">
              <div className="bg-white px-4 py-2 rounded-full shadow-sm border border-gray-200">
                <span className="text-sm font-medium text-gray-700">🏭 Real Asset Tokenization</span>
              </div>
              <div className="bg-white px-4 py-2 rounded-full shadow-sm border border-gray-200">
                <span className="text-sm font-medium text-gray-700">💸 Instant Global Payments</span>
              </div>
              <div className="bg-white px-4 py-2 rounded-full shadow-sm border border-gray-200">
                <span className="text-sm font-medium text-gray-700">📊 Unified Portfolio</span>
              </div>
            </div>
          </div>

          {/* Live Status Indicator */}
          <div className="flex justify-center mb-8">
            <div className="inline-flex items-center px-6 py-3 rounded-full bg-green-50 text-green-800 border border-green-200 shadow-sm">
              <div className="w-3 h-3 bg-green-500 rounded-full mr-3 animate-pulse"></div>
              <span className="font-semibold">Live Demo • Real Blockchain Integration</span>
            </div>
          </div>

          {/* Main Interface */}
          <div className="max-w-7xl mx-auto">
            <Tabs defaultValue="tokenization" className="w-full">
              <TabsList className="grid w-full grid-cols-4 mb-8 h-14 bg-white shadow-lg border border-gray-200/50 rounded-2xl p-2">
                <TabsTrigger value="tokenization" className="flex items-center space-x-2 font-semibold rounded-xl data-[state=active]:shadow-lg">
                  <span>🏭</span>
                  <span className="hidden sm:inline">Tokenize Assets</span>
                  <span className="sm:hidden">Assets</span>
                </TabsTrigger>
                <TabsTrigger value="assets" className="flex items-center space-x-2 font-semibold rounded-xl data-[state=active]:shadow-lg">
                  <span>📦</span>
                  <span className="hidden sm:inline">Portfolio</span>
                  <span className="sm:hidden">Portfolio</span>
                </TabsTrigger>
                <TabsTrigger value="payments" className="flex items-center space-x-2 font-semibold rounded-xl data-[state=active]:shadow-lg">
                  <span>💸</span>
                  <span className="hidden sm:inline">Payments</span>
                  <span className="sm:hidden">Pay</span>
                </TabsTrigger>
                <TabsTrigger value="wallet" className="flex items-center space-x-2 font-semibold rounded-xl data-[state=active]:shadow-lg">
                  <span>📊</span>
                  <span className="hidden sm:inline">Analytics</span>
                  <span className="sm:hidden">Stats</span>
                </TabsTrigger>
              </TabsList>

              <TabsContent value="tokenization" className="space-y-6">
                <AssetTokenization onTokenCreated={handleRefresh} />
              </TabsContent>

              <TabsContent value="assets" className="space-y-6">
                <AssetList refreshTrigger={refreshTrigger} />
              </TabsContent>

              <TabsContent value="payments" className="space-y-6">
                <PaymentInterface />
              </TabsContent>

              <TabsContent value="wallet" className="space-y-6">
                <WalletInfo refreshTrigger={refreshTrigger} />
              </TabsContent>
            </Tabs>
          </div>

          {/* Features Grid */}
          <div className="mt-20 grid md:grid-cols-3 gap-8">
            <div className="bg-white p-8 rounded-2xl shadow-sm border border-gray-100 hover:shadow-md transition-shadow">
              <div className="w-14 h-14 bg-gradient-to-br from-blue-500 to-blue-600 rounded-xl flex items-center justify-center mb-6">
                <svg className="w-7 h-7 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1" />
                </svg>
              </div>
              <h3 className="text-xl font-semibold text-gray-900 mb-3">Asset Tokenization</h3>
              <p className="text-gray-600 leading-relaxed">
                Convert real-world assets into SPL tokens with rich metadata storage. 
                Create fractionalized ownership of real estate, commodities, invoices, and more.
              </p>
            </div>

            <div className="bg-white p-8 rounded-2xl shadow-sm border border-gray-100 hover:shadow-md transition-shadow">
              <div className="w-14 h-14 bg-gradient-to-br from-green-500 to-green-600 rounded-xl flex items-center justify-center mb-6">
                <svg className="w-7 h-7 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 9V7a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2m2 4h10a2 2 0 002-2v-6a2 2 0 00-2-2H9a2 2 0 00-2 2v6a2 2 0 002 2zm7-5a2 2 0 11-4 0 2 2 0 014 0z" />
                </svg>
              </div>
              <h3 className="text-xl font-semibold text-gray-900 mb-3">Global Payments</h3>
              <p className="text-gray-600 leading-relaxed">
                Send USDC and SPL tokens globally with instant settlement. 
                Low fees, high speed, and complete transparency on Solana blockchain.
              </p>
            </div>

            <div className="bg-white p-8 rounded-2xl shadow-sm border border-gray-100 hover:shadow-md transition-shadow">
              <div className="w-14 h-14 bg-gradient-to-br from-purple-500 to-purple-600 rounded-xl flex items-center justify-center mb-6">
                <svg className="w-7 h-7 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
                </svg>
              </div>
              <h3 className="text-xl font-semibold text-gray-900 mb-3">Portfolio Management</h3>
              <p className="text-gray-600 leading-relaxed">
                Track all your tokenized assets in one unified dashboard. 
                Real-time analytics, transaction history, and portfolio insights.
              </p>
            </div>
          </div>

          {/* Stats Section */}
          <div className="mt-20 bg-gradient-to-r from-blue-600 to-purple-600 rounded-3xl p-8 text-white">
            <h2 className="text-3xl font-bold text-center mb-8">
              🏆 Real Blockchain Activity
            </h2>
            <div className="grid md:grid-cols-3 gap-8 text-center">
              <div>
                <div className="text-4xl font-bold mb-2">14+</div>
                <div className="text-blue-100">SPL Tokens Created</div>
              </div>
              <div>
                <div className="text-4xl font-bold mb-2">0.24+</div>
                <div className="text-blue-100">SOL Gas Consumed</div>
              </div>
              <div>
                <div className="text-4xl font-bold mb-2">15+</div>
                <div className="text-blue-100">Confirmed Transactions</div>
              </div>
            </div>
            <div className="mt-8 text-center">
              <a 
                href="https://explorer.solana.com/?cluster=devnet" 
                target="_blank" 
                rel="noopener noreferrer"
                className="inline-flex items-center px-6 py-3 bg-white bg-opacity-20 hover:bg-opacity-30 text-white rounded-xl transition-all duration-200"
              >
                Verify on Solana Explorer
                <svg className="w-5 h-5 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
                </svg>
              </a>
            </div>
          </div>

          {/* Instructions */}
          <div className="mt-16 bg-white rounded-2xl p-8 shadow-sm border border-gray-100">
            <h2 className="text-2xl font-bold text-gray-900 mb-6 text-center">
              🚀 How to Get Started
            </h2>
            <div className="grid md:grid-cols-4 gap-6">
              <div className="text-center">
                <div className="w-12 h-12 bg-blue-100 rounded-full flex items-center justify-center mx-auto mb-4">
                  <span className="text-xl font-bold text-blue-600">1</span>
                </div>
                <h3 className="font-semibold text-gray-900 mb-2">Connect Wallet</h3>
                <p className="text-sm text-gray-600">Connect your Phantom or Solflare wallet to start</p>
              </div>
              <div className="text-center">
                <div className="w-12 h-12 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-4">
                  <span className="text-xl font-bold text-green-600">2</span>
                </div>
                <h3 className="font-semibold text-gray-900 mb-2">Create Tokens</h3>
                <p className="text-sm text-gray-600">Tokenize your first asset with metadata</p>
              </div>
              <div className="text-center">
                <div className="w-12 h-12 bg-purple-100 rounded-full flex items-center justify-center mx-auto mb-4">
                  <span className="text-xl font-bold text-purple-600">3</span>
                </div>
                <h3 className="font-semibold text-gray-900 mb-2">Send Payments</h3>
                <p className="text-sm text-gray-600">Transfer tokens or USDC globally</p>
              </div>
              <div className="text-center">
                <div className="w-12 h-12 bg-orange-100 rounded-full flex items-center justify-center mx-auto mb-4">
                  <span className="text-xl font-bold text-orange-600">4</span>
                </div>
                <h3 className="font-semibold text-gray-900 mb-2">Track Portfolio</h3>
                <p className="text-sm text-gray-600">Monitor your assets and analytics</p>
              </div>
            </div>
          </div>
        </main>
      </div>
    </WalletConnectionProvider>
  );
}
