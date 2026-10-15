// Golden Phase SDK for C# — v0.1
//
// Copyright (c) 2026 LightSpeed Studio LLC. All Rights Reserved.
// P/Invoke wrapper over goldenphase.dll.
//
// Usage:
//   var engine = new GoldenPhase.Engine();
//   ulong body = engine.CreateBody();
//   engine.Tick(10);
//   var state = engine.State(body);

using System;
using System.Runtime.InteropServices;

namespace GoldenPhase
{
    public enum Result : uint
    {
        Ok = 0,
        MissingLaw = 1,
        Refused = 2,
        InvalidBody = 3,
        InvalidState = 4,
        BoundaryRefusal = 5,
        IllegalTransition = 6,
        NotConnected = 253,
        ProtocolError = 254,
        InternalError = 255,
    }

    public enum Knob : byte
    {
        X = 0, Q = 1, P = 2, L = 3,
        E = 4, Phi = 5, M = 6, H = 7,
    }

    public enum Provenance : byte
    {
        Derived = 0, Declared = 1, Measured = 2,
        LawDefined = 3, MissingLaw = 4, Refused = 5,
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct State
    {
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = 3)]
        public byte[] X;
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = 4)]
        public byte[] Q;
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = 3)]
        public byte[] P;
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = 3)]
        public byte[] L;
        public byte Energy;
        public byte Phase;
        public byte Mode;
        public byte History;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct Hamiltonian
    {
        public double HFree;
        public double G8Inverse;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct TemporalClock
    {
        public ulong Tick;
        public byte Phase;
        public byte Crossing;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct KernelView
    {
        public double Phi;
        public double Alpha;
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = 4)]
        public double[] Roots;
    }

    public class GoldenPhaseException : Exception
    {
        public Result ErrorCode { get; }
        public string Feature { get; }

        public GoldenPhaseException(Result code, string feature = "")
            : base($"Golden Phase error: {code}" +
                   (string.IsNullOrEmpty(feature) ? "" : $" ({feature})"))
        {
            ErrorCode = code;
            Feature = feature;
        }
    }

    internal static class Native
    {
        private const string Lib = "goldenphase";

        [DllImport(Lib)] public static extern Result gp_engine_create(out ulong engine);
        [DllImport(Lib)] public static extern Result gp_engine_destroy(ulong engine);
        [DllImport(Lib)] public static extern Result gp_body_create(ulong engine, out ulong body);
        [DllImport(Lib)] public static extern Result gp_body_remove(ulong engine, ulong body);

        [DllImport(Lib)] public static extern Result gp_command_phase(
            ulong engine, ulong body, byte knob, byte lane, byte phase);
        [DllImport(Lib)] public static extern Result gp_apply_impulse(
            ulong engine, ulong body, double x, double y, double z);
        [DllImport(Lib)] public static extern Result gp_apply_torque_impulse(
            ulong engine, ulong body, double x, double y, double z);

        [DllImport(Lib)] public static extern Result gp_tick(
            ulong engine, uint ticks, out ulong outTick);

        [DllImport(Lib)] public static extern Result gp_body_state(
            ulong engine, ulong body, out State state);
        [DllImport(Lib)] public static extern Result gp_read_hamiltonian(
            ulong engine, ulong body, out Hamiltonian h);
        [DllImport(Lib)] public static extern Result gp_read_position(
            ulong engine, ulong body,
            [MarshalAs(UnmanagedType.LPArray, SizeConst = 3)] byte[] roots);

        [DllImport(Lib)] public static extern Result gp_temporal_clock(
            ulong engine, out TemporalClock clock);
        [DllImport(Lib)] public static extern Result gp_temporal_seconds(
            ulong engine, out double seconds);

        [DllImport(Lib)] public static extern Result gp_em_charge_force(
            int qA, int qB, byte gA, byte gB, out sbyte force);

        [DllImport(Lib)] public static extern Result gp_kernel_view(out KernelView kv);

        [DllImport(Lib)] public static extern IntPtr gp_last_missing_feature();
        [DllImport(Lib)] public static extern IntPtr gp_runtime_version();
        [DllImport(Lib)] public static extern uint gp_abi_version();
        [DllImport(Lib)] public static extern uint gp_kernel_schema_version();
    }

    public class Engine : IDisposable
    {
        private ulong _handle;
        private bool _disposed;

        public Engine()
        {
            Check(Native.gp_engine_create(out _handle));
        }

        public void Dispose()
        {
            if (!_disposed && _handle != 0)
            {
                Native.gp_engine_destroy(_handle);
                _disposed = true;
            }
            GC.SuppressFinalize(this);
        }

        ~Engine() { Dispose(); }

        public ulong CreateBody()
        {
            Check(Native.gp_body_create(_handle, out ulong body));
            return body;
        }

        public void RemoveBody(ulong body)
            => Check(Native.gp_body_remove(_handle, body));

        public void CommandPhase(ulong body, Knob knob, byte lane, byte phase)
            => Check(Native.gp_command_phase(_handle, body, (byte)knob, lane, phase));

        public void ApplyImpulse(ulong body, double x, double y, double z)
            => Check(Native.gp_apply_impulse(_handle, body, x, y, z));

        public void ApplyTorqueImpulse(ulong body, double x, double y, double z)
            => Check(Native.gp_apply_torque_impulse(_handle, body, x, y, z));

        public ulong Tick(uint count = 1)
        {
            Check(Native.gp_tick(_handle, count, out ulong tick));
            return tick;
        }

        public State GetState(ulong body)
        {
            Check(Native.gp_body_state(_handle, body, out State s));
            return s;
        }

        public Hamiltonian GetHamiltonian(ulong body)
        {
            Check(Native.gp_read_hamiltonian(_handle, body, out Hamiltonian h));
            return h;
        }

        public TemporalClock GetTemporalClock()
        {
            Check(Native.gp_temporal_clock(_handle, out TemporalClock tc));
            return tc;
        }

        public double GetTemporalSeconds()
        {
            Check(Native.gp_temporal_seconds(_handle, out double s));
            return s;
        }

        public static sbyte EmChargeForce(int qA, int qB, byte gA, byte gB)
        {
            Check(Native.gp_em_charge_force(qA, qB, gA, gB, out sbyte f));
            return f;
        }

        public static KernelView GetKernelView()
        {
            Check(Native.gp_kernel_view(out KernelView kv));
            return kv;
        }

        public static string RuntimeVersion()
        {
            IntPtr p = Native.gp_runtime_version();
            return p != IntPtr.Zero ? Marshal.PtrToStringAnsi(p) : "unknown";
        }

        public static uint AbiVersion() => Native.gp_abi_version();
        public static uint KernelSchemaVersion() => Native.gp_kernel_schema_version();

        private static void Check(Result r)
        {
            if (r != Result.Ok)
            {
                IntPtr p = Native.gp_last_missing_feature();
                string feat = p != IntPtr.Zero ? Marshal.PtrToStringAnsi(p) : "";
                throw new GoldenPhaseException(r, feat);
            }
        }
    }
}
