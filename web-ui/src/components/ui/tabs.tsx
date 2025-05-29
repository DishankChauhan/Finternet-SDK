"use client"

import * as React from "react"
import { cn } from "@/lib/utils"

const Tabs = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement>
>(({ className, ...props }, ref) => (
  <div
    ref={ref}
    className={cn("w-full", className)}
    {...props}
  />
))
Tabs.displayName = "Tabs"

const TabsList = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement>
>(({ className, ...props }, ref) => (
  <div
    ref={ref}
    className={cn(
      "inline-flex items-center justify-center rounded-xl bg-gray-100 p-1 text-gray-600 shadow-sm border border-gray-200",
      className
    )}
    {...props}
  />
))
TabsList.displayName = "TabsList"

const TabsTrigger = React.forwardRef<
  HTMLButtonElement,
  React.ButtonHTMLAttributes<HTMLButtonElement> & {
    value: string
  }
>(({ className, value, children, ...props }, ref) => {
  const context = React.useContext(TabsContext)
  const isActive = context?.value === value

  return (
    <button
      ref={ref}
      className={cn(
        "inline-flex items-center justify-center whitespace-nowrap rounded-lg px-4 py-2.5 text-sm font-semibold transition-all duration-200 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
        isActive
          ? "bg-white text-gray-900 shadow-md border border-gray-200"
          : "text-gray-600 hover:text-gray-900 hover:bg-gray-50",
        className
      )}
      onClick={() => context?.setValue?.(value)}
      {...props}
    >
      {children}
    </button>
  )
})
TabsTrigger.displayName = "TabsTrigger"

const TabsContent = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement> & {
    value: string
  }
>(({ className, value, children, ...props }, ref) => {
  const context = React.useContext(TabsContext)
  const isActive = context?.value === value

  if (!isActive) return null

  return (
    <div
      ref={ref}
      className={cn(
        "mt-2 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2",
        className
      )}
      {...props}
    >
      {children}
    </div>
  )
})
TabsContent.displayName = "TabsContent"

// Context for managing tab state
const TabsContext = React.createContext<{
  value: string
  setValue: (value: string) => void
} | null>(null)

// Wrapper component to provide context
export function TabsProvider({ 
  defaultValue, 
  children 
}: { 
  defaultValue: string
  children: React.ReactNode 
}) {
  const [value, setValue] = React.useState(defaultValue)
  
  return (
    <TabsContext.Provider value={{ value, setValue }}>
      <Tabs>
        {children}
      </Tabs>
    </TabsContext.Provider>
  )
}

// Updated Tabs component that includes the provider
const TabsWithProvider = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement> & {
    defaultValue: string
  }
>(({ defaultValue, children, ...props }, ref) => (
  <TabsProvider defaultValue={defaultValue}>
    <div ref={ref} {...props}>
      {children}
    </div>
  </TabsProvider>
))
TabsWithProvider.displayName = "TabsWithProvider"

export { TabsWithProvider as Tabs, TabsList, TabsTrigger, TabsContent } 