import {
  ArrowLeftIcon,
  CheckIcon,
  XIcon,
  PaperPlaneTiltIcon,
  RobotIcon,
  UserIcon,
  ShieldIcon,
  InfoIcon,
  LockSimpleIcon,
  HandIcon,
} from "@phosphor-icons/react"
import { motion, AnimatePresence } from "motion/react"
import { useEffect, useRef, useState } from "react"
import { useNavigate, useParams } from "react-router"

import { Button, Textarea } from "@nexus/ui"
import { cn } from "@nexus/utils"

import { useSession } from "@/application/auth/use-session"
import { useTicket } from "@/application/tickets/use-ticket"
import { useTicketMessages } from "@/application/tickets/use-ticket-messages"
import { useSendMessage } from "@/application/tickets/use-send-message"
import {
  useApproveAi,
  useAssignTicket,
  useRejectAi,
  useUpdateTicketStatus,
} from "@/application/tickets/use-ticket-actions"
import { useTicketSse } from "@/application/tickets/use-ticket-sse"
import { StatusBadge } from "@/presentation/components/status-badge"
import { PriorityBadge } from "@/presentation/components/priority-badge"
import type { TicketMessage, TicketStatus } from "@/domain/tickets/ticket"
import { paths } from "@/presentation/router/paths"

function timeAgo(iso: string) {
  const diff = Date.now() - new Date(iso).getTime()
  const m = Math.floor(diff / 60000)
  if (m < 1) return "just now"
  if (m < 60) return `${m}m ago`
  const h = Math.floor(m / 60)
  if (h < 24) return `${h}h ago`
  return `${Math.floor(h / 24)}d ago`
}

function SenderIcon({ type }: { type: string }) {
  const cls = "h-3 w-3"
  if (type === "ai") return <RobotIcon className={cls} />
  if (type === "agent") return <ShieldIcon className={cls} />
  if (type === "system") return <InfoIcon className={cls} />
  return <UserIcon className={cls} />
}

function MessageBubble({
  msg,
  myId,
  index,
}: {
  msg: TicketMessage
  myId: string | undefined
  index: number
}) {
  const isMine = msg.senderId === myId
  const isAi = msg.senderType === "ai"
  const isSystem = msg.senderType === "system"

  if (isSystem) {
    return (
      <motion.div
        initial={{ opacity: 0, y: 4 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: index * 0.03 }}
        className="flex justify-center"
      >
        <span className="font-mono text-[10px] text-(--muted) bg-(--surface-2) border border-(--border) rounded-sm px-3 py-1">
          ⚙ {msg.content}
        </span>
      </motion.div>
    )
  }

  if (isAi) {
    return (
      <motion.div
        initial={{ opacity: 0, x: -8 }}
        animate={{ opacity: 1, x: 0 }}
        transition={{ delay: index * 0.03, duration: 0.2 }}
        className="flex gap-2.5 max-w-[85%]"
      >
        <div className="shrink-0 mt-0.5 h-6 w-6 rounded-sm bg-(--accent)/15 border border-(--accent)/30 flex items-center justify-center text-(--accent)">
          <RobotIcon className="h-3 w-3" />
        </div>
        <div>
          <div className="rounded-sm rounded-tl-none border border-(--accent)/30 bg-(--accent)/5 px-3 py-2.5">
            <p className="font-mono text-xs text-(--fg) whitespace-pre-wrap leading-relaxed">
              {msg.content}
            </p>
          </div>
          <div className="mt-1 flex items-center gap-1.5">
            <span className="font-mono text-[10px] text-(--accent) font-medium">
              AI assistant
            </span>
            <span className="text-(--border)">·</span>
            <span className="font-mono text-[10px] text-(--border)">
              {timeAgo(msg.createdAt)}
            </span>
          </div>
        </div>
      </motion.div>
    )
  }

  return (
    <motion.div
      initial={{ opacity: 0, x: isMine ? 8 : -8 }}
      animate={{ opacity: 1, x: 0 }}
      transition={{ delay: index * 0.03, duration: 0.2 }}
      className={cn("flex gap-2.5 max-w-[85%]", isMine && "self-end flex-row-reverse")}
    >
      <div
        className={cn(
          "shrink-0 mt-0.5 h-6 w-6 rounded-sm flex items-center justify-center",
          isMine
            ? "bg-(--surface-2) border border-(--border) text-(--muted)"
            : msg.senderType === "agent"
              ? "bg-(--success)/15 border border-(--success)/30 text-(--success)"
              : "bg-(--surface-2) border border-(--border) text-(--muted)"
        )}
      >
        <SenderIcon type={msg.senderType} />
      </div>
      <div className={cn(isMine && "items-end flex flex-col")}>
        <div
          className={cn(
            "rounded-sm px-3 py-2.5 border",
            isMine
              ? "rounded-tr-none bg-(--accent) border-(--accent) text-(--accent-fg)"
              : msg.senderType === "agent"
                ? "rounded-tl-none bg-(--success)/5 border-(--success)/30 text-(--fg)"
                : "rounded-tl-none bg-(--surface-2) border-(--border) text-(--fg)"
          )}
        >
          <p className="font-mono text-xs whitespace-pre-wrap leading-relaxed">
            {msg.content}
          </p>
        </div>
        <div className="mt-1 flex items-center gap-1.5">
          <span
            className={cn(
              "font-mono text-[10px] font-medium",
              isMine ? "text-(--accent)" : "text-(--muted)"
            )}
          >
            {isMine ? "you" : msg.senderType}
          </span>
          <span className="text-(--border)">·</span>
          <span className="font-mono text-[10px] text-(--border)">
            {timeAgo(msg.createdAt)}
          </span>
        </div>
      </div>
    </motion.div>
  )
}

export function TicketDetailPage() {
  const { id } = useParams<{ id: string }>()
  const navigate = useNavigate()
  const user = useSession()
  const [content, setContent] = useState("")
  const bottomRef = useRef<HTMLDivElement>(null)

  const { data: ticket, isLoading: ticketLoading } = useTicket(id!)
  const { data: messages, isLoading: messagesLoading } = useTicketMessages(id!)
  const send = useSendMessage(id!)
  const approve = useApproveAi(id!)
  const reject = useRejectAi(id!)
  const updateStatus = useUpdateTicketStatus(id!)
  const assign = useAssignTicket(id!)

  useTicketSse(id!)

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" })
  }, [messages])

  const isAgent = user?.role === "agent" || user?.role === "admin"
  const assignedToMe = ticket?.assigneeId === user?.userId
  const canChat = ticket?.status !== "closed" && ticket?.status !== "resolved"
  const needsApproval = ticket?.status === "awaiting_agent_approval"
  const isProcessing = ticket?.status === "processing_ai"

  const handleSend = async () => {
    const trimmed = content.trim()
    if (!trimmed || send.isPending) return
    setContent("")
    await send.mutateAsync(trimmed)
  }

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter" && (e.ctrlKey || e.metaKey)) {
      e.preventDefault()
      handleSend()
    }
  }

  if (ticketLoading) {
    return (
      <div className="flex items-center justify-center min-h-[60vh]">
        <div className="font-mono text-xs text-(--muted) animate-pulse">
          loading ticket...
        </div>
      </div>
    )
  }

  if (!ticket) {
    return (
      <div className="flex flex-col items-center justify-center min-h-[60vh] gap-3">
        <p className="font-mono text-sm text-(--muted)">ticket not found</p>
        <Button variant="ghost" size="sm" onClick={() => navigate(paths.app.tickets)}>
          ← back to tickets
        </Button>
      </div>
    )
  }

  return (
    <div className="flex flex-col h-[calc(100vh-8rem)] max-w-3xl mx-auto">
      {/* Header */}
      <motion.div
        initial={{ opacity: 0, y: -8 }}
        animate={{ opacity: 1, y: 0 }}
        className="shrink-0 space-y-3 pb-4 border-b border-(--border)"
      >
        <button
          onClick={() => navigate(paths.app.tickets)}
          className="flex items-center gap-1.5 font-mono text-xs text-(--muted) hover:text-(--fg) transition-colors"
        >
          <ArrowLeftIcon className="h-3 w-3" />
          back to tickets
        </button>

        <div className="flex items-start justify-between gap-4">
          <div className="min-w-0 flex-1">
            <div className="flex items-center gap-2 mb-1 flex-wrap">
              <span className="font-mono text-[10px] text-(--muted)">
                #{ticket.id.slice(0, 8)}
              </span>
              <StatusBadge status={ticket.status as TicketStatus} />
              <PriorityBadge priority={ticket.priority} />
              {ticket.category && (
                <span className="font-mono text-[10px] text-(--muted) rounded-sm border border-(--border) px-1.5 py-0.5">
                  {ticket.category}
                </span>
              )}
              {isAgent && (
                <span
                  className={cn(
                    "font-mono text-[10px] rounded-sm border px-1.5 py-0.5",
                    ticket.assigneeId
                      ? assignedToMe
                        ? "border-(--success)/30 text-(--success)"
                        : "border-(--border) text-(--muted)"
                      : "border-(--border) text-(--border)"
                  )}
                >
                  {ticket.assigneeId
                    ? assignedToMe
                      ? "assigned to you"
                      : "assigned"
                    : "unassigned"}
                </span>
              )}
            </div>
            <h1 className="font-mono text-base font-semibold text-(--fg)">
              {ticket.title}
            </h1>
            <p className="mt-1 font-mono text-xs text-(--muted) leading-relaxed">
              {ticket.description}
            </p>
          </div>

          {/* Agent actions */}
          {isAgent && !needsApproval && (
            <div className="flex items-center gap-2 shrink-0">
              {!assignedToMe &&
                ticket.status !== "closed" &&
                ticket.status !== "resolved" && (
                  <Button
                    size="sm"
                    variant="outline"
                    onClick={() => assign.mutate()}
                    loading={assign.isPending}
                  >
                    <HandIcon className="h-3.5 w-3.5" />
                    {ticket.assigneeId ? "take over" : "claim"}
                  </Button>
                )}
              {ticket.status !== "closed" && ticket.status !== "resolved" && (
                <Button
                  size="sm"
                  variant="secondary"
                  onClick={() => updateStatus.mutate("closed")}
                  loading={updateStatus.isPending}
                >
                  <LockSimpleIcon className="h-3.5 w-3.5" />
                  close
                </Button>
              )}
              {ticket.status === "resolved" && (
                <Button
                  size="sm"
                  variant="secondary"
                  onClick={() => updateStatus.mutate("closed")}
                  loading={updateStatus.isPending}
                >
                  <LockSimpleIcon className="h-3.5 w-3.5" />
                  close
                </Button>
              )}
            </div>
          )}
        </div>

        {/* AI approval banner */}
        <AnimatePresence>
          {isAgent && needsApproval && (
            <motion.div
              initial={{ opacity: 0, height: 0 }}
              animate={{ opacity: 1, height: "auto" }}
              exit={{ opacity: 0, height: 0 }}
              className="overflow-hidden"
            >
              <div className="flex items-center justify-between gap-3 rounded-sm border border-orange-400/30 bg-orange-400/5 px-4 py-3">
                <div className="flex items-center gap-2">
                  <RobotIcon className="h-4 w-4 text-orange-400 shrink-0" />
                  <div>
                    <p className="font-mono text-xs font-medium text-orange-400">
                      AI response pending review
                    </p>
                    <p className="font-mono text-[10px] text-(--muted)">
                      Review the AI reply below and approve or reject it
                    </p>
                  </div>
                </div>
                <div className="flex items-center gap-2 shrink-0">
                  <Button
                    size="sm"
                    variant="secondary"
                    onClick={() => reject.mutate()}
                    loading={reject.isPending}
                    className="border-(--destructive)/40 text-(--destructive) hover:bg-(--destructive)/10"
                  >
                    <XIcon className="h-3.5 w-3.5" />
                    reject
                  </Button>
                  <Button
                    size="sm"
                    onClick={() => approve.mutate()}
                    loading={approve.isPending}
                    className="bg-(--success) hover:opacity-85"
                  >
                    <CheckIcon className="h-3.5 w-3.5" />
                    approve
                  </Button>
                </div>
              </div>
            </motion.div>
          )}
        </AnimatePresence>

        {/* AI processing indicator */}
        {isProcessing && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            className="flex items-center gap-2 rounded-sm border border-(--warning)/30 bg-(--warning)/5 px-4 py-2.5"
          >
            <RobotIcon className="h-3.5 w-3.5 text-(--warning) animate-pulse" />
            <p className="font-mono text-xs text-(--warning)">
              AI is generating a response...
            </p>
          </motion.div>
        )}
      </motion.div>

      {/* Message thread */}
      <div className="flex-1 overflow-y-auto py-4 space-y-4 flex flex-col">
        {messagesLoading ? (
          <div className="flex items-center justify-center py-8">
            <span className="font-mono text-xs text-(--muted) animate-pulse">
              loading messages...
            </span>
          </div>
        ) : !messages?.length ? (
          <div className="flex flex-col items-center justify-center py-8 text-center">
            <p className="font-mono text-xs text-(--muted)">no messages yet</p>
            <p className="font-mono text-[10px] text-(--border) mt-1">
              the conversation will appear here
            </p>
          </div>
        ) : (
          messages.map((msg, i) => (
            <MessageBubble
              key={msg.id}
              msg={msg}
              myId={user?.userId}
              index={i}
            />
          ))
        )}
        <div ref={bottomRef} />
      </div>

      {/* Message input */}
      <motion.div
        initial={{ opacity: 0, y: 8 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.2 }}
        className="shrink-0 border-t border-(--border) pt-4"
      >
        {canChat ? (
          <div className="flex gap-2">
            <Textarea
              value={content}
              onChange={(e) => setContent(e.target.value)}
              onKeyDown={handleKeyDown}
              placeholder="Type a message... (Ctrl+Enter to send)"
              rows={2}
              className="flex-1 resize-none text-xs"
            />
            <Button
              onClick={handleSend}
              disabled={!content.trim() || send.isPending}
              loading={send.isPending}
              className="self-end"
              size="sm"
            >
              <PaperPlaneTiltIcon className="h-3.5 w-3.5" />
            </Button>
          </div>
        ) : (
          <div className="flex items-center justify-center gap-2 py-3">
            <LockSimpleIcon className="h-3.5 w-3.5 text-(--muted)" />
            <span className="font-mono text-xs text-(--muted)">
              this ticket is {ticket.status} — no new messages
            </span>
          </div>
        )}
      </motion.div>
    </div>
  )
}
